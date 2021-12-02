use std::env;
use std::process;
use findrs::{ Config, run };

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    run(config).unwrap();
    // if let Err(error) = run(config) {
    //     eprintln!("application error: {}", error);
    //     process::exit(1);
    // }
}
