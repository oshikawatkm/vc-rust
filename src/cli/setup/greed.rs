use crate::cli::setup::traits::Response;
use crate::errors::Error;
use clap::{App, Arg,  SubCommand, ArgMatches};

use std::fmt;


pub struct GreedResponse {
  greed: String,
}

impl GreedResponse {
  fn new(greed: String) -> Self {
    GreedResponse { greed: greed }
  }
}

impl Response for GreedResponse {}

impl fmt::Display for GreedResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.greed,)
  }
}

pub struct GreedCommand {}

impl<'a> GreedCommand {
    pub fn execute(matches: &ArgMatches) -> Result<Box<dyn Response>, Error> {
      let name: &str = matches
        .value_of("name")
        .ok_or(Error::InvalidArgs("name is invalid.".to_string()))?;

      let greed = String::from("Hello ") + name;

      Ok(Box::new(GreedResponse::new(
        greed
      )))
    }

    pub fn args<'b>() -> App<'a, 'b> {
      SubCommand::with_name("greed").args(&[
        Arg::with_name("name")
        .long("name")
        .required(true)
        .takes_value(true)
        .help("name for Greeding"),
      ])
    }
}