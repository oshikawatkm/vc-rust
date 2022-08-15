use crate::cli::setup::traits::Response;
use crate::errors::Error;
use std::fmt;
use clap::{App, Arg, ArgMatches, SubCommand};
use ssi::vc::Credential;
use crate::cli::generate_vc_from_path_or_str;


pub struct SignVCResponse {
  signed_vc: Credential,
}

impl SignVCResponse {
  fn new(signed_vc: Credential) -> Self {
    SignVCResponse {
      signed_vc: signed_vc,
    }
  }
}

impl Response for SignVCResponse {}

impl fmt::Display for SignVCResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.signed_vc)
  }
}

pub struct SignVCCommand {}

impl<'a> SignVCCommand {
  pub fn execute(matches: &ArgMatches) -> Result<Box<dyn Response>, Error> {
    let privkey: &str = matches
      .value_of("privkey")
      .ok_or(Error::InvalidArgs("privkey".to_string()))?;
  
    let path = matches.value_of("path")
    .ok_or(Error::InvalidArgs("path".to_string()))?;

    let  vc_str = matches.value_of("vc")
    .ok_or(Error::InvalidArgs("vc".to_string()))?;
    
    let vc = generate_vc_from_path_or_str(path, vc_str).unwrap();
    let proof = vc.generate_proof(jwk, options, resolver);
    let signed_vc = vc.add_proof(privkey);
    
    Ok(Box::new(SignVCResponse::new(signed_vc)))
  }

  pub fn args<'b>() -> App<'a, 'b> {
    SubCommand::with_name("signvc").args(&[
      Arg::with_name("privkey")
        .long("privkey")
        .required(true)
        .takes_value(true)
        .help("private key(WIF format) to sign VC"),
      Arg::with_name("path")
        .long("path")
        .takes_value(true)
        .help("(Optional) The path of VC JSON file"),
      Arg::with_name("vc")
        .long("vc")
        .takes_value(true)
        .help("(Optional) Raw VC JSON String"),
    ])
  }
}