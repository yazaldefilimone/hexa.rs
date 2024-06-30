use clap::Arg;
use clap::Command;

pub fn command_line() -> clap::ArgMatches {
  let run_subcommand = Command::new("run")
    .about("run a lox file.")
    .arg(Arg::new("file").help("the lox file to execute.").required(true));
  let compile_subcommand = Command::new("compile")
    .about("compile a lox file to bytecode.")
    .arg(Arg::new("file").help("the lox file to compile.").required(true));

  let maches = Command::new("lox")
    .version("0.1.0")
    .author("Yazalde Filimone <yazaldefilimon@gmail.com>")
    .about("The Lox Language")
    .subcommand_required(false)
    .arg_required_else_help(false)
    .subcommand(run_subcommand)
    .subcommand(compile_subcommand)
    .get_matches();

  maches
}
