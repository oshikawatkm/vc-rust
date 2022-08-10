// use clap::ArgMatches;
// use crate::cli::setup::traits::Response;
// use crate::errors::Error;
// use std::fmt;
// use ssi::vc::Credential;

// pub struct ImportVCResponse {
//   vc: Credential
// }

// impl ImportVCResponse {
//   fn new(vc: Credential) -> Self {
//     ImportVCResponse {
//       vc: vc
//     }
//   }
// }

// impl Response for ImportVCResponse {}

// impl fmt::Display for ImportVCResponse {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "{}", self.vc)
//   }
// }

// pub struct ImportVCCommand {}

// impl<'a> ImportVCCommand {
//   pub fn execute(matches: &ArgMatches) -> Result<Box<dyn Response>, Error> {
//     let vc: Credential = 
//     Ok(Box::new(GenerateVCResponse::new(
//       vc: vc
//     )))
//   }
// }
