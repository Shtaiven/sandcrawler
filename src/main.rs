#![allow(unused)] // Remove when you're done!

use spacetraders_sdk::apis;
use spacetraders_sdk::models;
use std::env;
use std::fs;
use std::io::Write;
use std::str::FromStr;

fn to_faction(s: &str) -> Result<models::register_request::Faction, ()> {
    match s.to_uppercase().as_str() {
        "COSMIC" => Ok(models::register_request::Faction::Cosmic),
        "VOID" => Ok(models::register_request::Faction::Void),
        "GALACTIC" => Ok(models::register_request::Faction::Galactic),
        "QUANTUM" => Ok(models::register_request::Faction::Quantum),
        "DOMINION" => Ok(models::register_request::Faction::Dominion),
        _ => Err(()),
    }
}

fn print_help() -> bool {
    println!("This is a help message");
    true
}

async fn run_registration(args: Vec<String>) -> Result<bool, ()> {
    let symbol = if args.len() >= 3 { &args[2] } else { "" };
    let faction = if args.len() >= 4 {
        match to_faction(&args[3]) {
            Ok(faction) => faction,
            Err(_) => {
                println!("Invalid faction");
                return Ok(false);
            }
        }
    } else {
        models::register_request::Faction::default()
    };
    let email = if args.len() >= 5 { &args[4] } else { "" };

    if symbol.is_empty() {
        println!("Must provide a symbol with registration");
        return Ok(false);
    } else if symbol.len() < 3 || symbol.len() > 14 {
        println!("Symbol must be between 3 and 14 characters");
        return Ok(false);
    }

    let configuration = apis::configuration::Configuration::new();
    let register_request = models::RegisterRequest::new(faction, symbol.to_string());

    println!(
        "Registration with values\n\tsymbol:\t{symbol}\n\tfaction:\t{faction:?}\n\temail:\t{email}"
    );

    let registration_result = apis::default_api::register(&configuration, Some(register_request)).await;
    let registration = match registration_result {
        Ok(response) => response.data,
        Err(error) => {
            println!("Registration error: {:?}", error);
            return Ok(false);
        },
    };

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

async fn run_cli(token_file: &str) -> Result<bool, ()> {
    println!("Starting CLI with token file: {token_file}");
    let token = fs::read_to_string(token_file).expect("Couldn't read token file contents");

    // TODO: Turn this into a cli app
    let agent = apis::my_agent_details(&token).await?;
    println!("{:?}", agent);
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<(), ()> {
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
