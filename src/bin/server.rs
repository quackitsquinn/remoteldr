use log::info;
use remote_ldr::{proto::remote_loader_server::RemoteLoaderServer, server::RemoteServer};
use tonic::transport::Server;

// Clippy is falsely reporting this as dead code
#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We don't use try_init because this is a binary and there should really be no case where a logging facade has been set.
    let _ = env_logger::init();

    let addr = "[::1]:50051".parse().unwrap();
    let mut client = RemoteServer::default();

    info!("Binding to {}", addr);

    Server::builder()
        .add_service(RemoteLoaderServer::new(client))
        .serve(addr)
        .await?;

    Ok(())
}
