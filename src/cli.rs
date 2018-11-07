extern crate clap;
extern crate config;
extern crate dirs;
#[macro_use]
extern crate handlebars;
extern crate palette;
#[macro_use]
extern crate serde_json;
extern crate xdg;
use clap::{App, Arg};
use std::path::PathBuf;


mod cocotte;
mod tests;
use cocotte::*;

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
        .arg(Arg::with_name("v").short("v").multiple(true).help(
            "Sets the level of verbosity.",
        ))
        .arg(Arg::with_name("dry-run").short("r").help(
            "Do not perform anything. Will debug a little.",
        ))
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

    let format = get_format(matches.value_of("format"));

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
    render(format, source_string, hsl, verbose);
}
