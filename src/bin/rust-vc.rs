extern crate vc_rust;

use clap::App;
use vc_rust::cli::setup::greed::GreedCommand;
use vc_rust::cli::setup::create_key::CreateKeyCommand;
// use vc_rust::cli::setup::generate_vc::GenerateVCCommand;
use vc_rust::cli::setup::read_memo::ReadMemoCommand;
// use vc_rust::cli::setup::import_vc::ImportVCCommand;
// use vc_rust::cli::setup::sign_vc::SignVCCommand;
use vc_rust::cli::setup::traits::Response;
use vc_rust::errors::Error;


fn main () {
  let matches = App::new("Setup")
    .subcommand(GreedCommand::args())
    .subcommand(ReadMemoCommand::args())
    .subcommand(CreateKeyCommand::args())
    .get_matches();
  let result: Result<Box<dyn Response>, Error> = match matches.subcommand_name() {
    Some("greed") => GreedCommand::execute(
      matches
        .subcommand_matches("greed")
        .expect("invalid args"),
    ),
    Some("readmemo") => ReadMemoCommand::execute(
  matches
        .subcommand_matches("readmemo")
        .expect("invalid args"),
    ),
    Some("createkey") => CreateKeyCommand::execute(
      matches
          .subcommand_matches("createkey")
          .expect("invalid args"),
    ),
    // Some("generate-vc") => GenerateVCCommand::execute(
    //   matches.subcommand_name("generate-vc")
    //   .expect("invalid args")
    // ),
    // Some("import-vc") => ImportVCCommand::execute(
    //   matches.subcommand_name("import-vc")
    //   .expect("invalid args")
    // ),
    // Some("sign-vc") => SignVCCommand::execute(
    //   matches.subcommand_name("sign-vc")
    //   .expect("invalid args")
    // ),
    None => return println!("No subcommand was used"),
    _ => unreachable!(),
  };
  match result {
    Ok(response) => println!("{}", response),
    Err(e) => println!("{}", e),
  }
 }