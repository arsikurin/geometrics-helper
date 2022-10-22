use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authorize current client
    Auth(Auth),

    /// Add a new problem
    Add(Add),

    /// Edit an existing problem
    Edit(Edit),

    /// Delete a specific problem
    Delete(Delete),
}

#[derive(Args)]
pub struct Auth {
    #[arg(short, long, value_name = "TEXT")]
    pub login: String,

    #[arg(short, long, value_name = "TEXT")]
    pub password: String,
}

#[derive(Args)]
pub struct Add {
    /// Problem name
    #[arg(short, long, value_name = "TEXT")]
    pub name: String,

    /// Problem description
    #[arg(short, long, value_name = "TEXT")]
    pub description: String,

    /// Problem solution
    #[arg(short, long, value_name = "BASE64")]
    pub solution: String,
}

#[derive(Args)]
pub struct Edit {
    /// Problem ID
    #[arg(long, value_parser = clap::value_parser ! (u16).range(1..), value_name = "INT")]
    pub id: u16,

    /// Problem name
    #[arg(short, long, value_name = "TEXT")]
    pub name: Option<String>,

    /// Problem description
    #[arg(short, long, value_name = "TEXT")]
    pub description: Option<String>,

    /// Problem solution
    #[arg(short, long, value_name = "BASE64")]
    pub solution: Option<String>,
}

#[derive(Args)]
pub struct Delete {
    /// Problem ID
    #[arg(long, value_parser = clap::value_parser ! (u16).range(1..), value_name = "INT")]
    pub id: u16,
}