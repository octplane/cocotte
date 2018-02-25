extern crate clap;
extern crate config;
extern crate palette;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::env;
use std::path::PathBuf;

use clap::{App, Arg};

use palette::{Hsl, RgbHue};
use palette::rgb::Rgb;
use palette::FromColor;

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

fn hsv(path: &str, black_list: &Vec<String>, verbose: u16) -> (u32, u32, u32) {
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
        0 => (0, 0, 0),
        _ => {
            let mut hue: f32 = 0.0;
            let saturation = 100.0 - 100.0 * (components.len() as f32).log(8.0);
            //println!("{:?}", components);

            for (ix, comp) in components.into_iter().enumerate() {
                match ix {
                    0 => {
                        hue = base_hue_for(comp);
                    }
                    _ => {
                        let sh = sub_hue_for(comp);
                        let delta = sh / (ix as i32) as f32;
                        hue = hue + delta;
                    }
                }
            }
            // Hue - 180 to 180
            let col = Hsl::new(RgbHue::from(hue), saturation / 100.0, 0.5);
            let rgbc: Rgb = Rgb::from_hsl(col);
            let r = (rgbc.red * 255.0) as u32;
            let g = (rgbc.green * 255.0) as u32;
            let b = (rgbc.blue * 255.0) as u32;
            (r, g, b)
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

fn base_hue_for(component: &str) -> f32 {
    let asa = component.as_bytes();

    let min: i32 = 97;
    let max: i32 = 122;
    let range = max - min;

    let mut count = 0;
    // 0-100
    let mut hue: i32 = 0;

    for cchar in asa[0..]
        .into_iter()
        .map(|c| *c as i32)
        .filter(|char| *char >= min && *char <= max)
    {
        hue = hue + (cchar - min) * 2 / (count + 1) ^ 2;
        count = count + 1;
    }
    if count == 0 {
        panic!("Ouch, no count for \"{}\"", component);
    }

    let mut out: f32 = hue as f32;
    out = 360.0 * out / (count * range) as f32;
    return out;
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
            Arg::with_name("INPUT")
                .help("Sets the input string to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let config_path = match matches.value_of("config") {
        Some(name) => Some(PathBuf::from(name)),
        None => match env::home_dir() {
            Some(mut hd) => {
                hd.push(".cocotterc.toml");
                Some(hd)
            }
            None => None,
        },
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

    let (r, g, b) = hsv(source_string, &black_list, verbose);

    if dry_run {
        println!("R:{} G:{} B:{}", r, g, b);
        println!("Hex: {:02X}{:02X}{:02X}", r, g, b);
    } else {
        // print!("\x1b]1337;SetColors=bg={:02X}{:02X}{:02X}\x07", r, g, b);
        print!("\x1b]6;1;bg;red;brightness;{}\x07", r);
        print!("\x1b]6;1;bg;green;brightness;{}\x07", g);
        print!("\x1b]6;1;bg;blue;brightness;{}\x07", b);
    }

    if false {
        let sample = vec![
            "aaaa",
            "aaaazzzz",
            "zzzzzzzz",
            "zzzzaaaa",
            "./rexif",
            "./rexif/target",
            "./rexif/.git",
            "./rexif/src",
            "./uu.js",
            "./uu.js/app",
            "./uu.js/config",
            "./uu.js/bower_components",
            "./uu.js/node_modules",
            "./uu.js/compiled",
            "./uu.js/public",
            "./uu.js/.git",
            "./imgui-rs",
            "./imgui-rs/imgui-glium-renderer",
            "./imgui-rs/target",
            "./imgui-rs/imgui-sys",
            "./imgui-rs/imgui-examples",
            "./imgui-rs/.git",
            "./imgui-rs/imgui-gfx-renderer",
            "./imgui-rs/src",
            "./rust-reverse-geocoder",
            "./rust-reverse-geocoder/target",
            "./rust-reverse-geocoder/.git",
            "./rust-reverse-geocoder/src",
            "./fsevent-rust",
            "./fsevent-rust/target",
            "./fsevent-rust/fsevent-sys",
            "./fsevent-rust/tests",
            "./fsevent-rust/examples",
            "./fsevent-rust/.git",
            "./fsevent-rust/src",
            "./TrLaFr",
            "./TrLaFr/digd",
            "./TrLaFr/databases",
            "./TrLaFr/docker",
            "./TrLaFr/importer",
            "./TrLaFr/importer-ng",
            "./TrLaFr/viewer-ng",
            "./TrLaFr/server-ng",
            "./TrLaFr/server",
            "./TrLaFr/gce",
            "./TrLaFr/t_index",
            "./TrLaFr/database-loader",
            "./TrLaFr/.git",
            "./TrLaFr/.vscode",
            "./TrLaFr/tantivy_server",
            "./TrLaFr/items-test",
            "./ansible_stdout_compact_logger",
            "./ansible_stdout_compact_logger/callbacks",
            "./ansible_stdout_compact_logger/test-files",
            "./ansible_stdout_compact_logger/.git",
            "./docker-rust",
            "./docker-rust/.git",
            "./cocotte",
            "./cocotte/target",
            "./cocotte/.git",
            "./cocotte/src",
            "./pitocools.rs",
            "./pitocools.rs/target",
            "./pitocools.rs/.git",
            "./pitocools.rs/src",
            "./photo-map",
            "./photo-map/app",
            "./photo-map/target",
            "./photo-map/.git",
            "./photo-map/src",
            "./arduino-code",
            "./arduino-code/arduino-mk",
            "./arduino-code/bin",
            "./arduino-code/examples",
            "./arduino-code/.git",
            "./tlfi-data",
            "./tlfi-data/imgs",
            "./tlfi-data/items",
            "./tlfi-data/assets",
            "./reese_tag_sync",
            "./Telebot",
            "./Telebot/node_modules",
            "./Telebot/.git",
            "./mon-mail-pro",
            "./mon-mail-pro/static-site",
            "./mon-mail-pro/frontend",
            "./mon-mail-pro/backend-rails",
            "./dockerfiles",
            "./dockerfiles/rails_app",
            "./dockerfiles/basebox",
            "./dockerfiles/trusty_ssh",
            "./dockerfiles/.git",
            "./dockerfiles/rbenv",
            "./gists",
            "./gists/d1a64f2724e9c74407b6de37f745f4e9",
            "./tlfi-scraper",
            "./tlfi-scraper/tlfi",
            "./tlfi-scraper/.scrapy",
            "./tlfi-scraper/bigs_ones",
            "./tlfi-scraper/.git",
            "./pitocools",
            "./pitocools/.git",
            "./pitocools/pitocools",
            "./pitocools/src",
        ];

        for v in sample {
            hsv(v, &black_list, verbose);
        }
    }
}
