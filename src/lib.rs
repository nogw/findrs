use std::fs;
use std::env;
mod ui;

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

pub struct LineSearch<'a> {
    pub line_number: usize,
    pub result: &'a str,
}

#[derive(Default)]
pub struct Search<'a> {
    pub file: &'a str,
    pub matches: usize,
    pub results: Vec<LineSearch<'a>>,
}

impl<'a> Search<'a> {
  pub fn get(query: &str, filename: &'a str, contents: &'a str) -> Search<'a> {
    let mut result = Search { file: filename, matches: 0, results: Vec::<LineSearch<'a>>::new() };
    
    for (index, line) in contents.lines().enumerate() {
      if line.contains(query) {
        result.matches += line.matches(query).count();

        result.results.push(
            LineSearch {
                line_number: index,
                result: line,
            }
        );
      }
    }
  
    result
  }
}

pub fn get_files(paths: fs::ReadDir, files: &mut Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
  for path in paths {    
    let p = path.unwrap().path();
    let file_type = fs::metadata(&p).unwrap().file_type();
    println!("{:?}", file_type);

    if file_type.is_dir() {
      get_files(fs::read_dir(&p).unwrap(), files);
    } else {
      files.push(p);
    }
  }
  
  files.to_vec()
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let files = get_files(
    fs::read_dir(std::path::PathBuf::from(&config.directory)).unwrap(),
    &mut Vec::<std::path::PathBuf>::new(),
  );

  println!("{}", ui::format_header(&config.query, &config.directory));

  for file in files {
    let contents = fs::read_to_string(&file)?;
    let results = Search::get(&config.query, file.to_str().unwrap(), &contents);

    if results.results.len() > 0 {
      println!("{}", ui::format_file_name(results.file, results.matches));
  
      for line in results.results {
        let text = ui::format_line_result(line.line_number, line.result, &config.query);
        println!("{}", text)
      }

      println!();
    }
  }

  Ok(())
}