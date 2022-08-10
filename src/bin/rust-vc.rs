extern crate vc_rust;

use clap::App;
use vc_rust::cli::setup::greed::GreedCommand;
use vc_rust::cli::setup::traits::Response;
use vc_rust::errors::Error;


fn main () {
  let matches = App::new("Setup")
    .subcommand(GreedCommand::args())
    .get_matches();
  let result: Result<Box<dyn Response>, Error> = match matches.subcommand_name() {
    Some("greed") => GreedCommand::execute(
      matches
        .subcommand_matches("greed")
        .expect("invalid args")
    ),
    None => return println!("No subcommand was used"), _ => unreachable!(),
  };
  match result {
    Ok(response) => println!("{}", response),
    Err(e) => println!("{}", e),
  }
 }