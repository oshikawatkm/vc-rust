use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Memo {
  pub content: String
}

// pub fn wite_file() -> Result<Memo, std::io::Error> {

// }

pub fn read_file(path: &str) -> Result<Memo, crate::errors::Error> {
  let json_file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(path)
    .expect(&format!("Failed to open {}", path));
  let reader = BufReader::new(json_file);
  let memo: Memo = serde_json::from_reader(reader).unwrap();
  Ok(memo)
}