fn print_green(text: &str) -> String {
  format!("{}{}{}{}{}", 
    termion::color::Fg(termion::color::Green), termion::style::Underline,
    text,
    termion::style::Reset, termion::color::Fg(termion::color::Reset),
  )
}

fn print_underline_purple(text: &str) -> String {
  format!("{}{}{}{}{}",
    termion::color::Fg(termion::color::Magenta),
    termion::style::Underline,
    text,
    termion::color::Fg(termion::color::Reset),
    termion::style::Reset
  )
}

pub fn format_error() -> String {
  format!(
    r#"{}usage: findrs {} {} {}{}: file or folder to search {}: word to search, if it is a sentence use " " e.g "a b c"{}: file extension to filter {}"#,
    "\n",
    print_green("<directory>"),
    print_green("<query>"),
    print_green("<filter>\n"),
    print_green("\ndirectory"),
    print_green("\nquery"),
    print_green("\nfilter"),
    "\n"
  )
}

pub fn format_header(query: &str, directory: &str, matches: usize) -> String {
  format!(
    "\nSearch: {}\nDirectory: {}\nTotal of matches: {}\n{}",
    print_underline_purple(query),
    print_underline_purple(directory),
    print_underline_purple(&matches.to_string()),
    if matches == 0 { 
      print_underline_purple("\nNo file has a match ;(\n") 
    } else { 
      print_underline_purple("") 
    }
  )
}

pub fn format_line_result(ln: usize, lr: &str, word: &str) -> String {
  let splited_line: Vec<&str> = lr.trim().split(&word).collect();
  let formated = format!("{}", print_green(word));

  format!(
    "{line:<2} | {result}", line = ln, result = splited_line.join(&formated)
  )
}

pub fn format_file_name(filename: std::path::PathBuf, matches: usize) -> String {
  format!(
    "{}File: {}\nMatches: {}\n",
    "──────────────────────────────────────────────\n\n",
    print_green(filename.to_str().unwrap()),
    print_green(&matches.to_string()),
  )
}