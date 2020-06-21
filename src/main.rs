use serde::{Deserialize, Serialize};
use std::io::BufRead;
use serde_json::error::Error;

#[derive(Debug, Deserialize)]
struct InputItem {
    item_id: u64,
    resolved_url: String,
}

#[derive(Debug, Serialize)]
struct OutputItem {
    item_id: u64,
    resolved_url: String,
    html: String,
}

// Input: json pocket items
// Output: { id, url, html }
fn main() {
    let stdin = std::io::stdin();

    stdin.lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| serde_json::from_str(&l))
        .for_each(|i: Result<InputItem, Error>| println!("{:?}", i))
}

/*
Outstanding questions
1. do we care about order? Not really for uploading
2. do we want to pass things through readability?
3. What do we do with errors?
4. Where do we want to put the output?
 */