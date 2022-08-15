pub mod setup;
use crate::utils::read_file;

use ssi::vc::{Credential};

pub fn generate_vc_from_path_or_str(path: &str, vc_string: &str) -> Result<Credential, std::io::Error> {
  let vc = if path {
    let file_content = read_file(path).unwrap();
    Credential::from_json(&file_content).unwrap();
  } else if vc_string {
    Credential::from_json(vc_string).unwrap();
  } else {
    panic!("invalid vc parameter")
  };
  Ok(vc)
}