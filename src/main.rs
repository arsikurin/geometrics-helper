use std::process::exit;

use clap::Parser;
use confy;
use confy::ConfyError;

use crate::args::{Cli, Commands};

mod args;
mod utils;
mod types;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg: Result<types::MyConfig, ConfyError> = confy::load("geometrics-helper", "conf");

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.command {
        Some(Commands::Auth(args)) => {
            let resp = utils::obtain_token(args.login, args.password).await;
            println!("{:#?}", resp);
            if let Ok(data) = resp {
                if data.code != 200 {
                    println!("{}", data.message.unwrap_or("".into()));
                } else {
                    match confy::store("geometrics-helper", "conf", types::MyConfig { token: format!("Bearer {}", data.token.unwrap_or("".into())) }) {
                        Ok(..) => {
                            println!("Token saved in:");
                            println!("{:?}", confy::get_configuration_file_path("geometrics-helper", "conf").unwrap_or("lol".into()));
                        }
                        Err(e) => {
                            println!("ERROR: {:?}", e);
                            exit(0);
                        }
                    }
                }
            } else {
                println!("Error!");
                exit(0);
            }
        }
        Some(Commands::Add(args)) => {
            let resp = utils::put_problem(utils::get_token_from_conf(&cfg), args.name, args.description, args.solution).await;
            println!("{:#?}", resp);
            if let Ok(data) = resp {
                if data.code != 200 {
                    println!("{}", data.message.unwrap_or("".into()));
                } else {
                    println!("Problem created. ID={}", data.problem_id.unwrap_or(0));
                }
            } else {
                println!("Error!");
                exit(0);
            }
        }
        Some(Commands::Edit(args)) => {
            let resp = utils::patch_problem(utils::get_token_from_conf(&cfg), args.id, args.name, args.description, args.solution).await;
            println!("{:#?}", resp);
            if let Ok(data) = resp {
                if data.code != 200 {
                    println!("{}", data.message.unwrap_or("".into()));
                } else {
                    println!("Problem updated.");
                }
            } else {
                println!("Error!");
                exit(0);
            }
        }
        Some(Commands::Delete(args)) => {
            let resp = utils::delete_problem(utils::get_token_from_conf(&cfg), args.id).await;
            println!("{:#?}", resp);
            if let Ok(data) = resp {
                if data.code != 200 {
                    println!("{}", data.message.unwrap_or("".into()));
                } else {
                    println!("Problem removed. ID={}", data.problem_id.unwrap_or(0));
                }
            } else {
                println!("Error!");
                exit(0);
            }
        }
        None => {}
    }
}


