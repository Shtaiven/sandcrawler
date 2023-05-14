#![allow(unused)] // Remove when you're done!

pub mod spacetraders;

use reqwest::Error;
use spacetraders::api;
use spacetraders::types;
use std::env;
use std::fs;

fn print_help() {
    println!("This is a help message")
}

async fn start_registration() -> Result<(), Error> {
    println!("Registration started");
    Ok(())
}

async fn start_cli(token_file: &str) -> Result<(), Error> {
    println!("Starting cli with token file: {token_file}");
    let token = fs::read_to_string(token_file).expect("Couldn't read token file contents");

    // TODO: Turn this into a cli app
    let agent = api::my_agent_details(token).await?;
    println!("{:?}", agent);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    let _ = if args.len() > 2 {
        match args[1].as_str() {
            "--register" => start_registration().await?,
            "--token" => start_cli(args[2].as_str()).await?,
            _ => print_help(),
        }
    } else {
        print_help()
    };
    Ok(())
}
