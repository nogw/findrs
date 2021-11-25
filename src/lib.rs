use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
  pub filename: String,
  pub query: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("usage: findrs <filename: string> <query: string>")
    }

    let filename = args[1].clone();
    let query = args[2].clone();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { filename, query, case_sensitive })
  }
}


pub struct Store {
  pub key: String,
  pub value: Option<Box<Store>>
}

// -> Result<Store, Box<dyn Error>>

pub fn run(config: Config) {
  // use std::collections::HashMap;

  // let store: HashMap<String, Store> = HashMap::new();
  
  // let key = String::from("file");
  // let value = String::from("filename.txt");

  fn list_files(paths: fs::ReadDir) {
    for path in paths {
      let p = &path.unwrap();
  
      let metadata = fs::metadata(p.path());
      let file_type = metadata.unwrap().file_type();
  
      if file_type.is_dir() {
        list_files(fs::read_dir(p.path()).unwrap());
      } else {
        println!("file: {}", p.path().display());
      }
    }
  }

  let paths = fs::read_dir(config.filename).unwrap();
  list_files(paths)  
}