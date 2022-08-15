use crate::cli::setup::traits::Response;
use crate::errors::Error;
use std::fmt;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::cli::generate_vc_from_path_or_str;

use ssi::vc::{VerificationResult};
pub struct VerifyVCResponse {
  result: VerificationResult,
}

impl VerifyVCResponse {
  fn new(result: VerificationResult) -> Self {
    VerifyVCResponse {
      result: result,
    }
  }
}

impl Response for VerifyVCResponse {}

impl fmt::Display for VerifyVCResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.result)
  }
}

pub struct VerifyVCCommand {}

impl<'a> VerifyVCCommand {
  pub fn execute(matches: &ArgMatches) -> Result<Box<dyn Response>, Error> {
    let privkey: &str = matches
      .value_of("privkey")
      .ok_or(Error::InvalidArgs("privkey".to_string()))?;

    let path = matches.value_of("path")
    .ok_or(Error::InvalidArgs("path".to_string()))?;

    let  vc_str = matches.value_of("vc")
    .ok_or(Error::InvalidArgs("vc".to_string()))?;
    
    let vc = generate_vc_from_path_or_str(path, vc_str).unwrap();
    let result = vc.verify();
    
    Ok(Box::new(VerifyVCResponse::new(result)))
  }

  pub fn args<'b>() -> App<'a, 'b> {
    SubCommand::with_name("verifyvc").args(&[
      Arg::with_name("pubkey")
        .long("pubkey")
        .required(true)
        .takes_value(true)
        .help("public key(WIF format) to verify VC"),
      Arg::with_name("path")
        .long("path")
        .takes_value(true)
        .help("(Optional) The path of VC JSON file"),
      Arg::with_name("sigedvc")
        .long("sigedvc")
        .takes_value(true)
        .help("(Optional) Raw VC JSON String"),
    ])
  }
}