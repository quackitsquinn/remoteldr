use clap::Parser;
use remote_ldr::{Arguments, Command};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Arguments::parse();

    match args.command {
        Command::Server(server_args) => {
            remote_ldr::server::run_server(server_args).await?;
        }
        Command::Client => {
            todo!("Client not implemented yet");
        }
    }
    Ok(())
}
