use config;
use dirs;
use palette::{Hsl, RgbHue, Shade};
use palette::rgb::Rgb;
use palette::FromColor;
use std::path::PathBuf;
use std::iter;
use xdg::BaseDirectories;


pub fn read_settings(config_path: PathBuf, verbose: u16) -> Option<config::Config> {
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

pub trait ApplyHue {
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
            r,
            g,
            b,
            r,
            g,
            b,
            source
        );
    }
}

pub static FORMAT_ITERM_BG: &str = "iterm_bg";
pub static FORMAT_ITERM_TAB: &str = "iterm_tab";
pub static FORMAT_HTML: &str = "html";

pub fn get_applier<'a>(format: Option<&str>) -> &'a ApplyHue {
    match format {
        Some(t) => {
            match t {
                "iterm_bg" => &ItermTermColorer {},
                "iterm_tab" => &ItermTabColorer {},
                _ => &HtmlDebugOutputer {},
            }
        }
        None => &HtmlDebugOutputer {},
    }
}

pub fn black_list(config: Option<config::Config>) -> Vec<String> {
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

pub fn hsl(path: &str, black_list: &Vec<String>, verbose: u16) -> Hsl {
    let ascii_path = path.to_ascii_lowercase();
    let str_black_list: Vec<&str> = black_list.iter().map(|i| i.as_str()).collect();

    let cleaned_path: Vec<&str> = ascii_path
        .split('/')
        .filter(|it| it.len() > 0 && !str_black_list.contains(it))
        .collect();
    if verbose > 0 {
        println!("Path components after filtering: {:?}", cleaned_path);
    }

    Hsl::new(RgbHue::from(hue_for(cleaned_path.concat())), 100.0, 0.5)
}

pub fn hue_for(str: String) -> f32 {
    let mut hue = 0.0;

    for (ix, c) in str.as_bytes().into_iter().enumerate() {
        let uc = *c as usize;
        let (pos, tot) = position_for(uc);
        let factor = match ix {
            0...9 => 36.0,
            _ => 0.4,
        };

        hue = hue + factor * (pos as f32) / (tot as f32);
    }
    hue
}

// Return the char positiong
fn position_for(chr: usize) -> (usize, usize) {
    let allowed_ranges: Vec<Vec<usize>> = vec![
        vec![48, 58], // 0 to :
        vec![97, 123], // a-z
    ];

    let indexer: Vec<usize> = allowed_ranges
        .into_iter()
        .map(|range| {
            iter::repeat(0)
                .take(range[1] - range[0])
                .enumerate()
                .map(|(ix, _b)| range[0] + ix)
                .collect()
        })
        .flat_map(|s: Vec<usize>| s)
        .collect();

    (
        indexer.iter().position(|&x| x == chr).unwrap_or(0),
        indexer.len(),
    )
}

pub fn get_config_path(path: &str) -> Option<PathBuf> {
    let base_directories = BaseDirectories::new().ok()?;
    let clean_path = base_directories.find_config_file(path);
    let dirty_path = dirs::home_dir();
    clean_path.or(dirty_path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(position_for('0' as usize), (0, 36));
        assert_eq!(position_for('1' as usize), (1, 36));
        assert_eq!(position_for('z' as usize), (35, 36));

        // We should do a complete turn of the hue colorspace between A and Z.
        assert_eq!(hue_for(String::from("0")), 0.0);
        // assert_eq!(hue_for(String::from("z")), 350);
        assert_eq!(
            hue_for(String::from("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz")),
            360.11102
        );
        assert_eq!(
            hue_for(String::from("zzzzzzzzzz/zzz//zzzzzzzzzzzzzzzzzzzz")),
            358.94437
        );
    }
}
