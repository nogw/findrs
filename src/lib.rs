use std::{ fs, env, path };
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

#[derive(PartialEq)]
pub enum FileType {
  Folder,
  File,
  Symlink
}

impl FileType {
  fn from_path(p: &str) -> std::io::Result<FileType> {
    let filetype = fs::metadata(p)?.file_type();
    if filetype.is_dir()  { return Ok(FileType::Folder) }
    if filetype.is_file() { return Ok(FileType::File) }
    Ok(FileType::Symlink) 
  }
}

pub struct LineSearch {
  pub line_number: usize,
  pub result: String,
}

#[derive(Default)]
pub struct Search<'a> {
  pub file: &'a str,
  pub matches: usize,
  pub results: Vec<LineSearch>,
}

impl<'a> Search<'a> {
  pub fn get(query: &'a str, file: &'a str ) -> Search<'a> {
    let mut result = Search { file: file, ..Default::default() };
    let content = fs::read_to_string(file).unwrap();

    content
      .lines()
      .enumerate()
      .filter(|(_, l)| l.contains(query))
      .for_each(|(i, l)| {
        result.matches += l.matches(query).count();
        result.results.push( LineSearch { line_number: i, result: l.to_owned() } )
      });

    result
  }
}

pub fn get_files(paths: fs::ReadDir, files: &mut Vec<path::PathBuf>) -> Vec<path::PathBuf> {
  for path in paths {    
    let p = path.unwrap().path();
    let filetype = FileType::from_path(p.to_str().unwrap()).unwrap();

    if filetype == FileType::Folder {
      get_files(fs::read_dir(&p).unwrap(), files);
    } else 
    if filetype == FileType::File {
      files.push(p)
    }
  }
  
  files.to_vec()
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let filetype = FileType::from_path(&config.directory).unwrap();
  
  match filetype {
    FileType::Folder => {
      let files = get_files( fs::read_dir(path::PathBuf::from(&config.directory)).unwrap(), &mut Vec::<path::PathBuf>::new() );
    
      println!("{}", ui::format_header(&config.query, &config.directory));
  
      for file in files {
        let results = Search::get(&config.query, file.to_str().unwrap());
    
        if results.results.len() > 0 {
          println!("{}", ui::format_file_name(results.file, results.matches));
      
          for line in results.results {
            println!("{}", ui::format_line_result(line.line_number, &line.result, &config.query))
          }
    
          println!();
        }
      }
    }
    FileType::File => {
      let result = Search::get(&config.query, &config.directory);
      
      println!("{}", ui::format_file_name(result.file, result.matches));
      
      for line in result.results {
        println!("{}", ui::format_line_result(line.line_number, &line.result, &config.query))
      }
      
      println!();
    }
    _ => ()
  }

  Ok(())
}
