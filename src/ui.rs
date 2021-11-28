pub fn format_line_result(ln: usize, lr: &str, word: &str) -> String {
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

pub fn format_file_name(filename: &str) -> String {
  return format!(
      "{}File: {}{}{}", 
      "──────────────────────────────────────────────\n\n",
      termion::color::Fg(termion::color::Green), 
      filename, 
      termion::color::Fg(termion::color::Reset),
  )
}