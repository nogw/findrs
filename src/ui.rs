pub fn format_header(query: &str, directory: &str, matches: usize) -> String {
  fn color(text: &str) -> String {
    format!("{}{}{}{}{}",
      termion::color::Fg(termion::color::Magenta),
      termion::style::Underline,
      text,
      termion::color::Fg(termion::color::Reset),
      termion::style::Reset
    )
  }

  return format!(
    "\nSearch: {}\nDirectory: {}\nTotal of matches: {}\n",
    color(query),
    color(directory),
    color(&matches.to_string()),
  )
}

pub fn format_line_result(ln: usize, lr: &str, word: &str) -> String {
  let splited_line: Vec<&str> = lr.trim().split(&word).collect();
  let formated = format!(
    "{}{}{}{}{}",
    termion::color::Fg(termion::color::Green), termion::style::Underline,
    word,
    termion::style::Reset, termion::color::Fg(termion::color::Reset),
  );

  return format!(
    "{line:<2} | {result}", line = ln, result = splited_line.join(&formated)
  );
}

pub fn format_file_name(filename: std::path::PathBuf, matches: usize) -> String {
  return format!(
      "{}File: {}{}{}\nMatches: {}{}{}\n",
      "──────────────────────────────────────────────\n\n",
      termion::color::Fg(termion::color::Green),
      filename.to_str().unwrap(),
      termion::color::Fg(termion::color::Reset),
      termion::color::Fg(termion::color::Green),
      matches,
      termion::color::Fg(termion::color::Reset),
  )
}