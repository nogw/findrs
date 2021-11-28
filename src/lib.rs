use std::fs;
use std::env;

pub struct Config {
  pub directory: String,
  pub query: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("usage: findrs <filename: string> <query: string>")
    }
    
    let directory = args[1].clone();
    let query = args[2].clone();
    
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
    Ok(Config { directory, query, case_sensitive })
  }
}

#[derive(Debug)]
pub struct LineResultSearch<'a> {
    pub line_number: usize,
    pub result: &'a str,
}

#[derive(Default)]
pub struct SearchResult<'a> {
    pub file: &'a str,
    pub results: Vec<LineResultSearch<'a>>,
}

pub fn search<'a>(query: &str, filename: &'a str, contents: &'a str) -> SearchResult<'a> {
  let lines = Vec::<LineResultSearch<'a>>::new();
  let mut result = SearchResult { file: filename, results: lines  };
  
  for (index, line) in contents.lines().enumerate() {
    if line.contains(query) {
      result.results.push(
          LineResultSearch {
              line_number: index,
              result: line,
          }
      );
    }
  }

  result
}

pub fn list_files(paths: fs::ReadDir, files: &mut Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
  for path in paths {    
    let p = path.unwrap().path();
    let file_type = fs::metadata(&p).unwrap().file_type();
    
    if file_type.is_dir() {
      list_files(fs::read_dir(&p).unwrap(), files);
    } else {
      files.push(p);
    }
  }
  
  files.to_vec()
}

fn printc(ln: usize, lr: &str, word: &str) -> String {
  let splited_line: Vec<&str> = lr.trim().split(&word).collect();
  let formated = format!(
    "{}{}{query}{resetStl}{resetFb}", 
    termion::color::Fg(termion::color::Green), 
    termion::style::Underline, 
    query = word, 
    resetStl = termion::style::Reset,
    resetFb = termion::color::Fg(termion::color::Reset),
  ); 
  
  return format!(
    "{line:<2} | {result}", line = ln, result = splited_line.join(&formated)
  );
}

use ui;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let files = list_files(
    fs::read_dir(&config.directory).unwrap(),
    &mut Vec::<std::path::PathBuf>::new()
  );

  for file in files {
    let contents = fs::read_to_string(&file)?;
    let results = search(&config.query, file.to_str().unwrap(), &contents);

    if results.results.len() > 0 {
      println!(
        "\nFile: {}{}{}", 
        termion::color::Fg(termion::color::Green), 
        results.file, 
        termion::color::Fg(termion::color::Reset
      ));
      println!("──────────────────────────────────────────────");
  
      for line in results.results {
        let text = printc(line.line_number, line.result, &config.query);
        println!("{}", text)
      }
    }

    println!();
  }


  Ok(())
}