use std::io::prelude::*;
use std::{fs, path};
mod ui;

pub struct Config {
  pub directory: String,
  pub search: String,
  pub filter: Option<String>,
}

impl Config {
  pub fn new(directory: String, search: String, filter: Option<String>) -> Result<Self, String> {
    Ok(Config {
      directory,
      search,
      filter,
    })
  }
}

#[derive(PartialEq)]
pub enum FileType {
  Folder,
  File,
  Unknown,
}

impl FileType {
  fn from_path(p: &str) -> std::io::Result<Self> {
    let filetype = fs::metadata(p)?.file_type();
    if filetype.is_dir() {
      return Ok(FileType::Folder);
    }
    if filetype.is_file() {
      return Ok(FileType::File);
    }
    Ok(FileType::Unknown)
  }
}

pub struct LineSearch {
  pub line_number: usize,
  pub result: String,
}

#[derive(Default)]
pub struct Search {
  pub file: path::PathBuf,
  pub matches: usize,
  pub results: Vec<LineSearch>,
}

impl Search {
  pub fn find(search: &str, file: &path::PathBuf) -> Search {
    let mut result = Search {
      file: file.clone(),
      ..Default::default()
    };

    let mut content = fs::File::open(file).unwrap();
    let mut buf = vec![];
    content.read_to_end(&mut buf).unwrap();
    let contents = String::from_utf8_lossy(&buf);

    contents
      .lines()
      .enumerate()
      .filter(|(_, l)| l.contains(search))
      .for_each(|(i, l)| {
        result.matches += l.matches(search).count();
        result.results.push(LineSearch {
          line_number: i,
          result: l.to_owned(),
        })
      });

    result
  }
}

pub fn get_number_matches(s: &Vec<Search>) -> usize {
  return s.into_iter().fold(0, |count, value| count + value.matches);
}

pub fn get_files(
  paths: fs::ReadDir,
  files: &mut Vec<path::PathBuf>,
  filter: &Option<String>,
) -> Vec<path::PathBuf> {
  for path in paths {
    let p = path.unwrap().path();
    let filetype = FileType::from_path(p.to_str().unwrap()).unwrap();

    match filetype {
      FileType::Folder => {
        get_files(fs::read_dir(&p).unwrap(), files, filter);
      }
      FileType::File => files.push(p),
      _ => (),
    }
  }

  match filter {
    Some(f) => {
      let filters: Vec<&str> = f.split(",").collect();
      files
        .to_owned()
        .into_iter()
        .filter(|file| filters.contains(&file.extension().unwrap().to_str().unwrap()))
        .collect::<Vec<path::PathBuf>>()
    }
    None => files.to_vec(),
  }
}

pub fn extract_matches(files: Vec<path::PathBuf>, search: &str) -> Vec<Search> {
  files
    .iter()
    .map(|file| Search::find(search, file))
    .filter(|result| result.results.len() > 0)
    .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let filetype = FileType::from_path(&config.directory).unwrap();
  match filetype {
    FileType::Folder => {
      let files = get_files(
        fs::read_dir(path::PathBuf::from(&config.directory)).unwrap(),
        &mut Vec::<path::PathBuf>::new(),
        &config.filter,
      );
      let results = extract_matches(files, &config.search);
      let matches = get_number_matches(&results);

      ui::format_header(&config.search, &config.directory, matches);

      for result in results {
        ui::format_file_name(result.file, result.matches);

        for line in result.results {
          ui::format_line_result(line.line_number, &line.result, &config.search);
        }

        println!()
      }
    }

    FileType::File => {
      let result = Search::find(&config.search, &path::PathBuf::from(&config.directory));

      ui::format_file_name(result.file, result.matches);

      for line in result.results {
        ui::format_line_result(line.line_number, &line.result, &config.search);
      }

      println!();
    }

    _ => (),
  }

  Ok(())
}
