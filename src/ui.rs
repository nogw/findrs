use ansi_term::Colour;

fn color(text: &str, color: Colour) -> String {
  format!("{}", color.underline().paint(text))
}

pub fn format_header(query: &str, directory: &str, matches: usize) {
  println!(
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

pub fn format_line_result(ln: usize, lr: &str, word: &str) {
  let splited_line = lr
    .trim()
    .split(&word)
    .collect::<Vec<&str>>()
    .join(&color(word, Colour::Red));

  println!("{:<2} | {}", &ln.to_string(), splited_line)
}

pub fn format_file_name(filename: std::path::PathBuf, matches: usize) {
  println!(
    "{}File: {}\nMatches: {}\n",
    "──────────────────────────────────────────────\n\n",
    color(filename.to_str().unwrap(), Colour::Green),
    color(&matches.to_string(), Colour::Green),
  )
}
