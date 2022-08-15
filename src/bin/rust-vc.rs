extern crate vc_rust;

use clap::App;
use vc_rust::cli::setup::greed::GreedCommand;
use vc_rust::cli::setup::create_key::CreateKeyCommand;
// use vc_rust::cli::setup::generate_vc::GenerateVCCommand;
use vc_rust::cli::setup::read_memo::ReadMemoCommand;
use vc_rust::cli::setup::import_vc::ImportVCCommand;
use vc_rust::cli::setup::sign_vc::SignVCCommand;
use vc_rust::cli::setup::verify_vc::VerifyVCCommand;
use vc_rust::cli::setup::traits::Response;
use vc_rust::errors::Error;


fn main () {
  let matches = App::new("Setup")
    .subcommand(GreedCommand::args())
    .subcommand(ReadMemoCommand::args())
    .subcommand(CreateKeyCommand::args())
    .subcommand(ImportVCCommand::args())
    .subcommand(SignVCCommand::args())
    .subcommand(VerifyVCCommand::args())
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
    // Some("generatevc") => GenerateVCCommand::execute(
    //   matches.subcommand_matches("generatevc")
    //   .expect("invalid args")
    // ),
    Some("importvc") => ImportVCCommand::execute(
      matches.subcommand_matches("importvc")
      .expect("invalid args")
    ),
    Some("signvc") => SignVCCommand::execute(
      matches.subcommand_matches("signvc")
      .expect("invalid args")
    ),
    Some("signvc") => SignVCCommand::execute(
      matches.subcommand_matches("signvc")
      .expect("invalid args")
    ),
    None => return println!("No subcommand was used"),
    _ => unreachable!(),
  };
  match result {
    Ok(response) => println!("{}", response),
    Err(e) => println!("{}", e),
  }
 }