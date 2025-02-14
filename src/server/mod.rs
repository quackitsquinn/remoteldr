use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Mutex,
};

use gethostname::gethostname;
use log::info;
use tonic::{transport::Server, Request, Response, Status};

use crate::{
    args::ServerArgs,
    proto::{
        remote_loader_server::{RemoteLoader, RemoteLoaderServer},
        Architecture, Data, DataRequest, OperatingSystem, Process, SystemInfoResponse,
    },
};

mod exec;
mod res;

const TARGET: &str = include_str!(concat!(env!("OUT_DIR"), "/target"));

#[derive(Debug, Default)]
pub struct RemoteServer {
    resource_manager: Mutex<res::ResourceManager>,
    process_manager: Mutex<exec::ExecutionController>,
}

#[tonic::async_trait]
impl RemoteLoader for RemoteServer {
    async fn system_info(
        &self,
        _: Request<()>,
    ) -> Result<Response<SystemInfoResponse>, tonic::Status> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;

        let os_resp = match os {
            "linux" => OperatingSystem::Linux,
            "macos" => OperatingSystem::Macos,
            "windows" => OperatingSystem::Windows,
            _ => OperatingSystem::UnknownOs,
        };

        let arch_resp = match arch {
            "x86" => Architecture::X86,
            "x86_64" => Architecture::X8664,
            "aarch64" => Architecture::Arm64,
            "arm" => Architecture::Arm,
            _ => Architecture::UnknownArch,
        };

        Ok(Response::new(SystemInfoResponse {
            server_name: gethostname().to_string_lossy().to_string(),
            os: os_resp as i32,
            arch: arch_resp as i32,
            target_triple: TARGET.to_string(),
        }))
    }

    async fn send_data(&self, data: Request<Data>) -> Result<tonic::Response<()>, tonic::Status> {
        let data = data.into_inner();
        let resource_manager = self.resource_manager.lock().unwrap();

        log::info!("Writing {} bytes into {}", data.data.len(), data.filepath);

        match resource_manager.write_file(&data.filepath, &data.data) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                log::error!("Failed to write file: {}", e);
                Err(Status::internal("Failed to write file"))
            }
        }
    }

    async fn get_data(
        &self,
        data: Request<DataRequest>,
    ) -> Result<tonic::Response<Data>, tonic::Status> {
        let req = data.into_inner();
        let resource_manager = self.resource_manager.lock().unwrap();
        log::info!("Reading {}", req.filepath);
        match resource_manager.read_file(&req.filepath) {
            Ok(data) => Ok(Response::new(Data {
                filepath: req.filepath,
                data,
            })),
            Err(e) => {
                log::error!("Failed to read file: {}", e);
                Err(Status::internal("Failed to read file"))
            }
        }
    }

    async fn spawn_process(
        &self,
        command: Request<Process>,
    ) -> Result<tonic::Response<u32>, tonic::Status> {
        let command = command.into_inner();
        let mut proc_manager = self.process_manager.lock().unwrap();
        match proc_manager.spawn_process(&command) {
            Ok(output) => Ok(Response::new(output.id())),
            Err(e) => {
                log::error!("Failed to execute command: {}", e);
                Err(Status::internal("Failed to execute command"))
            }
        }
    }
}

pub async fn run_server(args: ServerArgs) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, args.port));
    info!("Starting server on {}", addr);
    let server = RemoteServer::default();
    server
        .resource_manager
        .lock()
        .unwrap()
        .set_working_dir(args.workdir.clone());
    info!("Working directory set to {}", args.workdir.display());

    Server::builder()
        .add_service(RemoteLoaderServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
