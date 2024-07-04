use std::fs::File;
use std::io::Read;

mod ast;
mod cli;
mod diagnostics;
mod evaluator;
mod parser;
mod repl;
mod scanner;
mod shared;
mod values;
mod visitor;

use cli::command_line;
use parser::Parser;
use repl::prompt;
use scanner::Scanner;
use visitor::print_visitor::PrintVisitor;

struct LoxFile {
  pathname: String,
  content: String,
}

fn read_file(file_name: &str) -> LoxFile {
  let mut file = File::open(file_name).unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  let pathname = file_name.to_string();
  return LoxFile { pathname, content };
}

pub fn execute(raw: &str, path_name: &str) {
  let mut scanner = Scanner::new(raw.to_string(), path_name);
  let mut parser = Parser::new(&mut scanner);
  let ast = parser.parse();
  let mut visitor = PrintVisitor;
  let result = ast.accept(&mut visitor);
  println!("{}", result);
}
// =====================
// run file(compile) file output result
//
fn run(file_name: &str) {
  let file = read_file(file_name);
  execute(&file.content, file_name)
}

// =====================
// compile file output bytecode
//
fn compile(file: &str) {}

fn main() {
  let matches = command_line();
  match matches.subcommand() {
    Some(("run", sub_matches)) => {
      let file = sub_matches.get_one::<String>("file").unwrap();
      run(file);
    }
    Some(("compile", sub_matches)) => {
      let file = sub_matches.get_one::<String>("file").unwrap();
      compile(file);
    }
    _ => prompt(),
  }
}
