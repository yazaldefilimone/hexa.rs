mod cli;
mod repl;
use cli::command_line;
use repl::prompt;

// =====================
// run file(compile) file output result
//
fn run(file: &str) {}

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
