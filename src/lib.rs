use std::{fs, path};
mod ui;

pub struct Config {
  pub directory: String,
  pub query: String,
  pub filter: Option<String>,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, String> {
    if args.len() < 3 {
      return Err(ui::format_error());
    }

    let directory = args[1].clone();
    let query = args[2].clone();
    let filter = if args.get(3) != None {
      Some(args[3].clone())
    } else {
      None
    };

    Ok(Config {
      directory,
      query,
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
  pub fn find(query: &str, file: &path::PathBuf) -> Search {
    let mut result = Search {
      file: file.clone(),
      ..Default::default()
    };

    fs::read_to_string(file)
      .unwrap()
      .lines()
      .enumerate()
      .filter(|(_, l)| l.contains(query))
      .for_each(|(i, l)| {
        result.matches += l.matches(query).count();
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
    Some(f) => files
      .to_owned()
      .into_iter()
      .filter(|file| file.extension().unwrap().to_str().unwrap() == f)
      .collect::<Vec<path::PathBuf>>(),
    None => files.to_vec(),
  }
}

pub fn extract_matches(files: Vec<path::PathBuf>, query: &str) -> Vec<Search> {
  files
    .iter()
    .map(|file| Search::find(query, file))
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
      let results = extract_matches(files, &config.query);
      let matches = get_number_matches(&results);

      println!(
        "{}",
        ui::format_header(&config.query, &config.directory, matches)
      );

      for result in results {
        println!("{}", ui::format_file_name(result.file, result.matches));

        for line in result.results {
          println!(
            "{}",
            ui::format_line_result(line.line_number, &line.result, &config.query)
          )
        }

        println!()
      }
    }

    FileType::File => {
      let result = Search::find(&config.query, &path::PathBuf::from(&config.directory));
      println!("{}", ui::format_file_name(result.file, result.matches));
      if result.results.len() > 0 {
        for line in result.results {
          println!(
            "{}",
            ui::format_line_result(line.line_number, &line.result, &config.query)
          )
        }
        println!();
      }
    }
    _ => (),
  }

  Ok(())
}
