use clap::ArgMatches;
use crate::cli::setup::traits::Response;
use crate::errors::Error;
use std::fmt;

pub struct ReadMemoResponse {
  memo: Memo
}

impl ReadMemoResponse {
  fn new(memo: Memo) -> Self {
    ReadMemoResponse {
      memo: memo
    }
  }
}

impl Response for ReadMemoResponse {}

impl fmt::Display for ReadMemoResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.memo)
  }
}

pub struct ReadMemoCommand {}

impl<'a> ReadMemoCommand {
  pub fn execute(matches: &ArgMatches) -> Result<Box<dyn Response>, Error> {
    let path: &str = matches
      .value_of("path")
      .ok_or(Error::InvalidArgs("path".to_string()));

    let memo = read_memo(path)?;
    Ok(Box::new(ReadMemoResponse::new(memo)))
  }

  pub fn args<'b>() -> App<'a, 'b> {
    SubCommand::with_name("read-memo").args(&[
      Args::with_name("path")
        .long("path")
        .required(true)
        .takes_value(true)
        .help("the path existing memo file.")
    ])
  }
}
