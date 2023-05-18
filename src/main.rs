#![allow(unused)] // Remove when you're done!

pub mod spacetraders;

use spacetraders_sdk::apis::Error;
use spacetraders::api;
use spacetraders::types;
use std::env;
use std::fs;
use std::io::Write;

fn print_help() -> bool {
    println!("This is a help message");
    true
}

async fn run_registration(args: Vec<String>) -> Result<bool, Error> {
    let symbol = if args.len() >= 3 { &args[2] } else { "" };
    let faction = if args.len() >= 4 { &args[3] } else { "COSMIC" };
    let email = if args.len() >= 5 { &args[4] } else { "" };

    if symbol.is_empty() {
        println!("Must provide a symbol with registration");
        return Ok(false);
    }

    println!(
        "Registration with values\n\tsymbol:\t{symbol}\n\tfaction:\t{faction}\n\temail:\t{email}"
    );

    let registration = api::register_new_agent(symbol, faction, email).await?;

    let token_file_path = format!("token_{symbol}.txt");

    let mut file_result = fs::File::create(&token_file_path);

    let mut file = match file_result {
        Ok(f) => f,
        Err(error) => return Ok(false),
    };
    file.write_all(registration.token.as_bytes());
    println!("Registration of {symbol} succeeded\nToken written to {token_file_path}");
    Ok(true)
}

async fn run_cli(token_file: &str) -> Result<bool, Error> {
    println!("Starting CLI with token file: {token_file}");
    let token = fs::read_to_string(token_file).expect("Couldn't read token file contents");

    // TODO: Turn this into a cli app
    let agent = api::my_agent_details(&token).await?;
    println!("{:?}", agent);
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<(), Error<>> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        match args[1].as_str() {
            "--register" => run_registration(args).await?,
            "--token" => run_cli(&args[2]).await?,
            _ => print_help(),
        }
    } else {
        print_help()
    };
    Ok(())
}
