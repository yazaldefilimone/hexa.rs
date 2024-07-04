use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::evaluator::Evaluator;
// use crate::execute;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::values::Value;
use crate::visitor::print_visitor::PrintVisitor;
// use crate::visitor::print_visitor::PrintVisitor;

fn run_code(raw: &str, scanner: &mut Scanner) -> Value {
  let mut parser = Parser::new(scanner);
  let ast = parser.parse();
  let mut visitor = PrintVisitor;
  let debug = ast.accept(&mut visitor);
  // println!("debug: {}", debug);
  let mut evaluator = Evaluator::new("REPL");
  let result = evaluator.evaluate(&ast);
  return result;
}

pub fn prompt() {
  let mut rl = DefaultEditor::new().unwrap();
  let warning_exit = "(To exit, press Ctrl+C again or Ctrl+D or type .exit)";
  let welcome = "Welcome to the Lox REPL!";
  let exit_commands = [".exit"];
  let prompt = ">> ";
  let mut ctrl_d = false;
  if rl.load_history("history.txt").is_err() {}

  println!("{}", welcome);
  loop {
    match rl.readline(prompt) {
      Ok(line) => {
        ctrl_d = false;
        if exit_commands.contains(&line.trim()) {
          break;
        }
        let mut scanner = Scanner::new(line.to_string(), "repl");
        if scanner.error_handler.had_error {
          scanner.error_handler.had_error = false;
          continue;
        }
        let result = run_code(&line, &mut scanner);
        println!("Result: {}", result);
        if scanner.error_handler.had_error {
          scanner.error_handler.had_error = false;
          continue;
        }
        rl.add_history_entry(line.as_str()).unwrap();
      }
      Err(ReadlineError::Interrupted) => {
        break;
      }
      Err(ReadlineError::Eof) => {
        if ctrl_d {
          break;
        }
        println!("{}", warning_exit);
        ctrl_d = true;
      }
      Err(err) => {
        println!("REPL ERROR: {:?}", err);
      }
    }
  }
  rl.save_history("history.txt").unwrap_or(());
}
