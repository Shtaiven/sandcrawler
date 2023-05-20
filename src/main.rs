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

fn print_help() -> () {
    println!("This is a help message");
}

async fn run_registration(args: Vec<String>) -> Result<(), ()> {
    let symbol = if args.len() >= 3 { &args[2] } else { "" };
    let faction = if args.len() >= 4 {
        match to_faction(&args[3]) {
            Ok(faction) => faction,
            Err(_) => {
                println!("Invalid faction");
                return Err(());
            }
        }
    } else {
        models::register_request::Faction::default()
    };
    let email = if args.len() >= 5 { &args[4] } else { "" };

    if symbol.is_empty() {
        println!("Must provide a symbol with registration");
        return Err(());
    } else if symbol.len() < 3 || symbol.len() > 14 {
        println!("Symbol must be between 3 and 14 characters");
        return Err(());
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
            return Err(());
        },
    };

    let token_file_path = format!("token_{symbol}.txt");

    let mut file_result = fs::File::create(&token_file_path);

    let mut file = match file_result {
        Ok(f) => f,
        Err(error) => {
            println!("Couldn't write token to file {}", token_file_path);
            return Err(());
        },
    };
    file.write_all(registration.token.as_bytes());
    println!("Registration of {symbol} succeeded\nToken written to {token_file_path}");
    Ok(())
}

async fn run_cli(token_file: &str) -> Result<(), ()> {
    println!("Starting CLI with token file: {token_file}");
    let token = fs::read_to_string(token_file).expect("Couldn't read token file contents");

    let mut configuration = apis::configuration::Configuration::new();
    configuration.bearer_access_token = Some(token);

    // TODO: Turn this into a cli app
    let agent_result = apis::agents_api::get_my_agent(&configuration).await;
    let agent = match agent_result {
        Ok(agent) => agent,
        Err(error) => {
            println!("Error while getting my agent: {}", error);
            return Err(());
        },
    };
    println!("{:?}", agent);
    Ok(())
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
