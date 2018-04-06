extern crate clap;
extern crate config;
extern crate palette;
extern crate xdg;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::env;
use std::path::PathBuf;
use std::iter;


use clap::{App, Arg};

use palette::{Hsl, RgbHue, Shade};
use palette::rgb::Rgb;
use palette::FromColor;

use xdg::BaseDirectories;

fn read_settings(config_path: PathBuf, verbose: u16) -> Option<config::Config> {
    let mut settings = config::Config::default();
    match settings.merge(config::File::from(config_path)) {
        Ok(config) => Some(config.clone()),
        Err(e) => {
            if verbose > 0 {
                println!("Error while reading the configuration: {}", e);
            }
            None
        }
    }
}

fn hsl_to_rgb(hue: Hsl) -> (u32, u32, u32) {
    let rgbc: Rgb = Rgb::from_hsl(hue);
    // this should be >= 0.0
    let r = (rgbc.red.max(0.0) * 255.0) as u32;
    let g = (rgbc.green.max(0.0) * 255.0) as u32;
    let b = (rgbc.blue.max(0.0) * 255.0) as u32;
    (r, g, b)
}

trait ApplyHue {
    fn apply(&self, source: &str, hue: Hsl, verbose: u16) {}
}


struct ItermTabColorer {}

impl ApplyHue for ItermTabColorer {
    fn apply(&self, _: &str, hue: Hsl, verbose: u16) {
        let (r, g, b) = hsl_to_rgb(hue);
        print!("\x1b]6;1;bg;red;brightness;{}\x07", r);
        print!("\x1b]6;1;bg;green;brightness;{}\x07", g);
        print!("\x1b]6;1;bg;blue;brightness;{}\x07", b);
    }
}

struct ItermTermColorer {}

impl ApplyHue for ItermTermColorer {
    fn apply(&self, _: &str, hue: Hsl, verbose: u16) {
        let my_hue = hue.darken(0.4);
        let (r, g, b) = hsl_to_rgb(my_hue);
        if verbose > 0 {
            println!("R:{}, G:{}, B:{}", r, g, b);
        }
        print!("\x1b]1337;SetColors=bg={:02X}{:02X}{:02X}\x07", r, g, b);
    }
}

struct HtmlDebugOutputer {}

impl ApplyHue for HtmlDebugOutputer {
    fn apply(&self, source: &str, hue: Hsl, verbose: u16) {
        let (r, g, b) = hsl_to_rgb(hue);
        println!(
            "<div style='background-color: #{:02X}{:02X}{:02X};'>{} {} {} {}</div>",
            r, g, b, r, g, b, source
        );
    }
}

static FORMAT_ITERM_BG: &str = "iterm_bg";
static FORMAT_ITERM_TAB: &str = "iterm_tab";
static FORMAT_HTML: &str = "html";

fn get_applier<'a>(format: Option<&str>) -> &'a ApplyHue {
    match format {
        Some(t) => match t {
            "iterm_bg" => &ItermTermColorer {},
            "iterm_tab" => &ItermTabColorer {},
            _ => &HtmlDebugOutputer {},
        },
        None => &HtmlDebugOutputer {},
    }
}

fn black_list(config: Option<config::Config>) -> Vec<String> {
    if let Some(config) = config {
        if let Ok(blacklist_config) = config.get_array("blacklist") {
            let bl: Result<Vec<String>, _> =
                blacklist_config.into_iter().map(|v| v.into_str()).collect();
            if let Ok(black_list) = bl {
                return black_list;
            }
        }
    }
    return vec![];
}

fn hsl(path: &str, black_list: &Vec<String>, verbose: u16) -> Hsl {
    let ascii_path = path.to_ascii_lowercase();
    let str_black_list: Vec<&str> = black_list.iter().map(|i| i.as_str()).collect();

    let components: Vec<&str> = ascii_path
        .split('/')
        .filter(|it| it.len() > 0 && !str_black_list.contains(it))
        .collect();
    if verbose > 0 {
        println!("Path components after filtering: {:?}", components);
    }

    match components.len() {
        0 => Hsl::new(RgbHue::from(0.0), 0.0, 0.0),
        _ => {
            let mut hue: f32 = 0.0;
            let saturation = 100.0 - 100.0 * (components.len() as f32).log(8.0);
            //println!("{:?}", components);

            for (ix, comp) in components.into_iter().enumerate() {
                match ix {
                    0 => {
                        hue = base_hue_for(comp, verbose);
                        if verbose > 0 {
                            println!("Base hue for {} is {}", comp, hue);
                        }
                    }
                    _ => {
                        let sh = sub_hue_for(comp);
                        let delta = sh / (ix as i32) as f32;
                        hue = hue + delta;
                    }
                }
            }
            // Hue - 180 to 180
            Hsl::new(RgbHue::from(hue), saturation / 100.0, 0.5)
        }
    }
}

fn sub_hue_for(component: &str) -> f32 {
    let min: i32 = 97;
    let max: i32 = 122;
    let mid: i32 = (max + min) as i32 / 2;

    let bytes = component.as_bytes();

    let selector: Option<i32> = bytes
        .into_iter()
        .map(|c| *c as i32)
        .filter(|char| *char > min && *char < max)
        .next();

    let mut hasher = DefaultHasher::new();
    hasher.write(bytes);

    let hv = hasher.finish() & 0xF;
    let delta: i32 = (15 - hv) as i32;
    let out: f32 = match selector {
        Some(c) => c - mid + delta,
        _ => 0,
    } as f32;
    return out;
}

#[test]
fn test() {
    assert_eq!(position_for('0' as usize), 0);
    assert_eq!(position_for('1' as usize), 1);
    assert_eq!(position_for('z' as usize), 35);
}

fn position_for(chr: usize) -> usize {
    let allowed_ranges: Vec<Vec<usize>> = vec![
        vec![48, 58], // 0 to :
        vec![97, 123], // a-z
    ];

    let indexer: Vec<usize> = allowed_ranges.into_iter().map(
        |range| {
            iter::repeat(0).take(range[1]- range[0]).enumerate().map( |(ix, _b)|
                range[0] + ix
                ).collect()
        }).flat_map(|s: Vec<usize>| s).collect();

    indexer.iter().position(|&x| x == chr).unwrap_or(0)
}

fn base_hue_for(component: &str, verbose: u16) -> f32 {
    let asa = component.as_bytes();

    let min: i32 = 97; // this is a
    let max: i32 = 122; // this is z
    let range = max - min + 1;
    println!("Range is {}", range);

    let mut count = 0;
    let mut hue: i32 = 0;

    for current_char in asa[0..]
        .into_iter()
        .map(|c| *c as i32)
        .filter(|char| *char >= min && *char <= max)
    {
        let char_pos = current_char - min;
        let delta = 360 * char_pos / range;
        let factor = 1 + 5 * count * count * count;
        if factor > range {
            break;
        }
        if verbose > 1 {
            println!(
                "Delta is {} for {}, factor {}: {}, {}",
                delta,
                current_char,
                factor,
                count,
                count * count
            );
        }

        hue = hue + delta / factor;
        count = count + 1;
    }
    if count == 0 {
        panic!("Ouch, no count for \"{}\"", component);
    }

    hue as f32
}

fn get_config_path(path: &str) -> Option<PathBuf> {
    let base_directories = BaseDirectories::new().ok()?;
    let clean_path = base_directories.find_config_file(path);
    let dirty_path = env::home_dir().map(|hd| hd.join(path));
    clean_path.or(dirty_path)
}

fn main() {
    let matches = App::new("Cocotte, the hue setter.")
        .version("1.0")
        .author("Pierre BAILLET <pierre@baillet.name>")
        .about("Colorize your iTerm tab/bg according to cwd")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file. Default to ~/.cocotterc.toml")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity."),
        )
        .arg(
            Arg::with_name("dry-run")
                .short("r")
                .help("Do not perform anything. Will debug a little."),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .takes_value(true)
                .possible_values(&[FORMAT_HTML, FORMAT_ITERM_BG, FORMAT_ITERM_TAB])
                .default_value(FORMAT_ITERM_TAB)
                .help("Output format."),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input string to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let config_path = match matches.value_of("config") {
        Some(name) => Some(PathBuf::from(name)),
        None => get_config_path(".cocotterc.toml"),
    };
    let dry_run = matches.is_present("dry-run");

    let verbose = match matches.occurrences_of("v") {
        1 => {
            println!("Verbose enabled.");
            1
        }
        2 => {
            println!("Tons of verbose info");
            2
        }
        3 => {
            println!("Don't be crazy");
            3
        }
        _ => 0,
    };

    let applier = get_applier(matches.value_of("format"));

    let config: Option<config::Config> = if let Some(path) = config_path {
        if verbose > 0 {
            println!("Reading configuration from: {:?}", path);
        }
        read_settings(path, verbose)
    } else {
        None
    };

    let black_list = black_list(config);

    if verbose > 0 {
        println!("Black list is: {:?}", black_list);
    }

    let source_string = matches.value_of("INPUT").unwrap();

    if verbose > 0 {
        println!("Using input string: {}", source_string);
    }

    let hsl = hsl(source_string, &black_list, verbose);
    applier.apply(source_string, hsl, verbose);
}
