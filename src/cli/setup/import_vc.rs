use crate::cli::setup::traits::Response;
use crate::errors::Error;
use crate::utils::read_file;
use std::fmt;
use clap::{App, Arg, ArgMatches, SubCommand};
use serde_json::{json, Value};
use ssi::vc::Credential;
use crate::cli::generate_vc_from_path_or_str;

pub struct ImportVCResponse {
  vc: Credential,
}

impl ImportVCResponse {
  fn new(vc: Credential) -> Self {
    ImportVCResponse {
      vc: vc,
    }
  }
}

impl Response for ImportVCResponse {}

impl fmt::Display for ImportVCResponse  {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.vc)
  }
}

pub struct ImportVCCommand {}

impl<'a> ImportVCCommand {
  pub fn execute(matches: &ArgMatches) -> Result<Box<dyn Response>, Error> {
    let path: &str = matches
      .value_of("path")
      .ok_or(Error::InvalidArgs("path".to_string()))?;

    let file_content = read_file(path).unwrap();
    let vc = Credential::from_json(&file_content).unwrap();
    Ok(Box::new(ImportVCResponse::new(vc)))
  }

  pub fn args<'b>() -> App<'a, 'b> {
    SubCommand::with_name("importvc").args(&[
      Arg::with_name("path")
        .long("path")
        .required(true)
        .takes_value(true)
        .help("the path existing memo file.")
    ])
  }
}
