fn main() {
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
        hsv(v);
    }
}
    
fn hsv(source: &str) {
    let asacii = source.to_ascii_lowercase();
    let asa = asacii.as_bytes();

    let min: u32 = 97;
    let max = 122;
    let mid = (max + min) / 2;
    let range = max - min;

    let hue_count = 3;
    let mut count = 0;
    let mut hue: u32 = 0;

    for a in asa[0..].into_iter() {
        let c = *a as char;
        let ia = *a as u32;
        if ia < min || ia > max {
            continue;
        }
        if count == hue_count {
            break;
        }
        hue = hue + (ia - min)*2/(count +1)^2;
        count = count + 1;
    }

    hue = 255 * hue / (hue_count * range);

    println!("<div style='background-color: hsl({}, 100%, 50%)'>{} {}</div>", hue, hue, asacii);
}
