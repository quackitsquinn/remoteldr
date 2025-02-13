use gethostname::gethostname;
use tonic::{Request, Response};

use crate::proto::{
    remote_loader_server::RemoteLoader, Architecture, OperatingSystem, SystemInfoResponse,
};

#[derive(Debug, Default)]
pub struct RemoteServer {}

#[tonic::async_trait]
impl RemoteLoader for RemoteServer {
    async fn system_info(
        &self,
        _: Request<()>,
    ) -> Result<tonic::Response<SystemInfoResponse>, tonic::Status> {
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
            // TODO: Arm and other architectures
            _ => Architecture::UnknownArch,
        };

        Ok(Response::new(SystemInfoResponse {
            server_name: gethostname().to_string_lossy().to_string(),
            os: os_resp as i32,
            arch: arch_resp as i32,
        }))
    }
}
