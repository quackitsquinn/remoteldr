use std::sync::Mutex;

use gethostname::gethostname;
use tonic::{Code, Request, Response, Status};

use crate::proto::{
    remote_loader_server::RemoteLoader, Architecture, Data, DataRequest, OperatingSystem,
    SystemInfoResponse,
};

mod exec;
mod res;

#[derive(Debug, Default)]
pub struct RemoteServer {
    resource_manager: Mutex<res::ResourceManager>,
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
}
