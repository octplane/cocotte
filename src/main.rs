extern crate palette;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::env;

use palette::{Hsl, RgbHue};
use palette::rgb::Rgb;
use palette::FromColor;

fn main() {
    // Prints each argument on a separate line
    let black_list = vec!["users", "pierrebaillet", ".", "Documents", "src", "datadog", "mine"];

    if env::args().len() > 1 {
        let path: String = env::args().last().unwrap();
        hsv(path.as_str(), &black_list);

    } else {
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
            hsv(v, &black_list);
        }
    }
}


fn hsv(path: &str, black_list: &Vec<&str>) {
    let ascii_path = path.to_ascii_lowercase();
    let components: Vec<&str> = ascii_path
        .split('/')
        .filter(|it| it.len() > 0 && !black_list.contains(it))
        .collect();
    let mut hue: f32 = 0.0;
    let saturation = 100.0 - 100.0 * (components.len() as f32).log(8.0);
    //println!("{:?}", components);

    for (ix, comp) in components.into_iter().enumerate() {
        match ix  {
            0 => { 
                hue = base_hue_for(comp);
            },
            _ => {
                let sh = sub_hue_for(comp);
                let delta = sh / (ix as i32) as f32;
                hue = hue + delta ;
            }
        }
    }
    // Hue - 180 to 180
    let col = Hsl::new(RgbHue::from(hue), saturation / 100.0, 0.5);
    let rgbc: Rgb = Rgb::from_hsl(col);
    let r = (rgbc.red * 255.0) as i32;
    let g = (rgbc.green * 255.0) as i32;
    let b = (rgbc.blue * 255.0) as i32;


    print!("\x1b]6;1;bg;red;brightness;{}\x07", r);
    print!("\x1b]6;1;bg;green;brightness;{}\x07", g);
    print!("\x1b]6;1;bg;blue;brightness;{}\x07", b);
}

fn sub_hue_for(component: &str) -> f32 {
    let min: i32 = 97;
    let max: i32 = 122;
    let mid: i32 = (max + min) as i32 / 2;

    let bytes = component
        .as_bytes();

    let selector: Option<i32> = bytes
        .into_iter()
        .map(|c| *c as i32)
        .filter(|char| *char > min && *char < max )
        .next();

    let mut hasher = DefaultHasher::new();
    hasher.write(bytes);

    let hv = hasher.finish() & 0xF;
    let delta: i32 =  (15 - hv) as i32;
    let out: f32 = match selector {
        Some(c) => c - mid + delta,
        _ => 0
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

    for cchar in asa[0..].into_iter().map(|c| *c as i32).filter(|char| *char >= min && *char <= max ) {
        hue = hue + (cchar - min)*2/(count +1)^2;
        count = count + 1;
    }
    if count == 0 {
        panic!("Ouch, no count for \"{}\"", component);
    }

    let mut out:f32 = hue as f32;
    out = 360.0 * out / (count * range) as f32;
    return out;
}
