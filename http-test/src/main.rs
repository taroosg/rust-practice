fn main() {
  // let hoge = get_http();
  get_http();
  println!("Hello");
}

struct Res {
  total: usize,
  items: HashMap<String, String>,
  pages: usize,
}

use std::collections::HashMap;
extern crate serde;
extern crate serde_json;
use serde_json::Value;

#[tokio::main]
async fn get_http() -> Result<(), Box<dyn std::error::Error>> {
  let resp = reqwest::get("https://api.fbi.gov/wanted/v1/list")
    .await?
    .text()
    .await?;
  let hoge: Value = serde_json::from_str(&resp).unwrap();
  dbg!(&hoge["items"]);
  Ok(())
}
