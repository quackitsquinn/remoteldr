mod args;
pub mod client;
pub mod server;
pub use args::*;
pub mod proto {
    tonic::include_proto!("remoteldr");
}
