use clap::{App, Arg};

use findrs::{run, Config};
use std::process;

fn main() {
    let matches = App::new("findrs")
        .version("1.0")
        .about("findrs - like grep")
        .arg("<FILE> 'file to use'")
        .arg("<TEXT> 'text to search'")
        .arg(
            Arg::new("filter")
                .about("file extension you want to use")
                .takes_value(true)
                .short('f')
                .long("filter")
                .value_name("FILTER"),
        )
        .get_matches();

    let file: String = matches.value_of_t("FILE").unwrap();
    let text: String = matches.value_of_t("TEXT").unwrap();
    let filter = matches.value_of("filter").map(|s| s.to_string());

    let config = Config::new(file, text, filter).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    run(config).unwrap();
}
