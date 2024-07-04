pub struct ErrorHandler {
  pub had_error: bool,
  pub path_name: String,
}

impl ErrorHandler {
  pub fn new(path_name: &str) -> Self {
    ErrorHandler { had_error: false, path_name: path_name.to_owned() }
  }

  pub fn report(&mut self, line: usize, _where: &str, message: &str) {
    /*
    error: Unexpected )
        at /Users/yazaldefilimone/www/learnspace/lox.rs/test.js:1:25
    */

    let report_message = format!("error: {}\n\tat {}, line: {}\n", message, self.path_name, line);
    println!("{}", report_message);
    self.had_error = true;
  }

  pub fn error(&mut self, line: usize, message: &str) {
    self.report(line, "<unknown>", message);
  }
}
