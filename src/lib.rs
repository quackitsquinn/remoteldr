mod args;
pub mod client;
pub mod server;
pub use args::*;

/// Protobuf / gRPC definitions
pub mod proto {
    tonic::include_proto!("remoteldr");
}
