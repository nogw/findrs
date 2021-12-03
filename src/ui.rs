use ansi_term::Colour;

fn color(text: &str, color: Colour) -> String {
  format!("{}", color.underline().paint(text))
}

pub fn format_error() -> String {
  format!(
    r#"{}usage: findrs {} {} {}{}: file or folder to search {}: word to search, if it is a sentence use " " e.g "a b c"{}: file extension to filter {}"#,
    "\n",
    color("<directory>", Colour::Green),
    color("<query>", Colour::Green),
    color("<filter>", Colour::Green),
    color("\ndirectory", Colour::Green),
    color("\nquery", Colour::Green),
    color("\nfilter", Colour::Green),
    "\n"
  )
}

pub fn format_header(query: &str, directory: &str, matches: usize) -> String {
  format!(
    "\nSearch: {}\nDirectory: {}\nTotal of matches: {}\n{}",
    color(query, Colour::Purple),
    color(directory, Colour::Purple),
    color(&matches.to_string(), Colour::Purple),
    if matches == 0 {
      color("\nNo file has a match ;(\n", Colour::Purple)
    } else {
      color("", Colour::Purple)
    }
  )
}

pub fn format_line_result(ln: usize, lr: &str, word: &str) -> String {
  let splited_line: Vec<&str> = lr.trim().split(&word).collect();

  format!(
    "{:<2} | {}",
    &ln.to_string(),
    splited_line.join(&color(word, Colour::Red))
  )
}

pub fn format_file_name(filename: std::path::PathBuf, matches: usize) -> String {
  format!(
    "{}File: {}\nMatches: {}\n",
    "──────────────────────────────────────────────\n\n",
    color(filename.to_str().unwrap(), Colour::Green),
    color(&matches.to_string(), Colour::Green),
  )
}
