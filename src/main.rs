use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::BufRead;
use serde_json::error::Error;
use futures::prelude::*;

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
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();

    let client = Client::builder().build()?;
    let output = futures::future::join_all(stdin.lock()
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| serde_json::from_str(&l))
        .flat_map(|r: Result<InputItem, Error>| r.ok())
        .map(|i| {
            client.get(&i.resolved_url)
                .send()
                .and_then(|r| r.text())
                .map(|r| match r {
                    Ok(t) => OutputItem {
                        item_id: i.item_id,
                        resolved_url: i.resolved_url,
                        html: t,
                    },
                    _ => panic!("uh oh"),
                })
                .map(|o| serde_json::to_string(&o))
        })
    ).await;
    output.iter()
        .flat_map(|o| o.as_ref().ok())
        .for_each(|o| println!("{:?}", o));
    Ok(())
}

/*
Outstanding questions
1. do we care about order? Not really for uploading
2. do we want to pass things through readability?
3. What do we do with errors?
4. Where do we want to put the output?
 */