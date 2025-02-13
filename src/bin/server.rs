use remote_ldr::{proto::remote_loader_server::RemoteLoaderServer, server::RemoteServer};
use tonic::transport::Server;

// Clippy is falsely reporting this as dead code
#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let mut client = RemoteServer::default();

    Server::builder()
        .add_service(RemoteLoaderServer::new(client))
        .serve(addr)
        .await?;

    Ok(())
}
