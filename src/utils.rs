use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Memo {
  content: String
}

pub fn wite_file() -> Result<<Memo>, std::io::Error> {

}

pub fn read_file(path: &str) -> Result<<Memo>, std::io::Error> {
  let mut json_file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(path)
    .expect(&format!("Failed to open {}", path));
  let reader = BufferReader::new(json_file);
  let memos: Memos = serde_json::from_reader(reader)?;
  Ok(memo)
}