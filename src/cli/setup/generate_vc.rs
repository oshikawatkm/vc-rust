// use clap::ArgMatches;
// use crate::cli::setup::traits::Response;
// use crate::errors::Error;
// use std::fmt;
// use ssi::vc::Credential;


// pub struct GenerateVCResponse {
//   vc: Credential
// }

// impl GenerateVCResponse {
//   fn new(vc: Credential) -> Self {
//     GenerateVCResponse {
//       vc: vc
//     }
//   }
// }

// impl Response for GenerateVCResponse {}

// impl fmt::Display for GenerateVCResponse {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "{}", self.vc)
//   }
// }

// pub struct GenerateVCCommand {}

// // impl<'a> GenerateVCCommand {
// //   pub fn execute(matches: &ArgMatches) -> Result<Box<dyn Response>, Error> {
// //     let vc: Credential = 
// //     Ok(Box::new(GenerateVCResponse::new(
// //       vc: vc
// //     )))
// //   }
// // }
