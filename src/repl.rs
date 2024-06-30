use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

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
        rl.add_history_entry(line.as_str()).unwrap();
        println!("Result: {}", line);
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
