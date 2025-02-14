use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Run the server
    Server(ServerArgs),
    /// Run the client
    Client,
}

#[derive(Debug, Clone, Args)]
pub struct ServerArgs {
    /// The port to listen on
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
    /// The workdir to use
    #[arg(short, long, default_value = "remoteldr")]
    pub workdir: PathBuf,
}
