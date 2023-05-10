#![allow(unused)] // Remove when you're done!

pub mod spacetraders;

use reqwest::Error;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let token_file = &args[1];
    println!("token file: {token_file}");
    let token = fs::read_to_string(token_file)
        .expect("Couldn't read token file contents");
    let agent = spacetraders::api::get_agent(token).await?;
    println!("{:?}", agent);
    Ok(())
}
