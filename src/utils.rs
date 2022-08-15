use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use serde_json::{json, Value};


// pub fn wite_file() -> Result<Memo, std::io::Error> {

// }

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
  let json_file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(path)
    .expect(&format!("Failed to open {}", path));
  let reader = BufReader::new(json_file);
  let read_result: serde_json::Value = serde_json::from_reader(reader).unwrap();
  let result_string = read_result.to_string();
  Ok(result_string)
}