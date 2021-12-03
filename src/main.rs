use findrs::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    run(config).unwrap();
}
