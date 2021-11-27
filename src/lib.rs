use std::fs;
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

impl<'a> SearchResult<'a> {
    pub fn _new(file: &'a str, results: Vec<LineResultSearch<'a>>) -> SearchResult<'a> {
        SearchResult {
            file,
            results
        }
    }
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

// pub fn list_files<'a>(paths: fs::ReadDir, files: &mut Vec<&'a std::path::PathBuf>) -> Vec<&'a std::path::PathBuf> {
//   for path in paths {    
    
//     let file_type = fs::metadata(
//       path.unwrap().path()
//     ).unwrap().file_type();
    
//     if file_type.is_dir() {
//       list_files(
//         fs::read_dir(
//           path.unwrap().path()
//         ).unwrap(),
//         files
//       );
//     } else {
//       files.push(
//         path.unwrap().path()
//       );
//     }
//   }
  
//   files.to_vec()
// }

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  // let list = list_files(
  //   fs::read_dir(config.filename).unwrap(),
  //   &mut Vec::<&std::path::PathBuf>::new()
  // );


  fn printc(ln: usize, lr: &str, word: &str) -> String {
    let splited_line: Vec<&str> = lr.split(&word).collect();
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

  let contents = fs::read_to_string(&config.filename)?;
  let results = search(&config.query, &config.filename, &contents);

  println!("\nFile: {}{}{}", termion::color::Fg(termion::color::Green), results.file, termion::color::Fg(termion::color::Reset));
  println!("──────────────────────────────────────────────");
  for line in results.results {
    let text = printc(line.line_number, line.result, &config.query);
    println!("{}", text)
  }

  Ok(())
}