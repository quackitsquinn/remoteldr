use std::{
    path::{Path, PathBuf},
    process::Child,
};

use crate::proto::Process;

/// A controller for managing the execution of child processes.
#[derive(Debug)]
pub struct ExecutionController {
    pub processes: Vec<Child>,
}

impl ExecutionController {
    /// Creates a new `ExecutionController`.
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    /// Adds a process to the controller.
    pub fn add_process(&mut self, process: Child) {
        self.processes.push(process);
    }

    /// Removes a process from the controller.
    pub fn remove_process(&mut self, process: u32) -> Result<Child, ProcessError> {
        let index = self
            .processes
            .iter()
            .position(|p| p.id() == process)
            .ok_or(ProcessError::ProcessNotFound)?;

        Ok(self.processes.remove(index))
    }

    /// Kills all processes in the controller.
    pub fn kill_all(&mut self) -> Result<(), ProcessError> {
        while let Some(mut process) = self.processes.pop() {
            process.kill()?;
        }
        Ok(())
    }

    /// Kills a specific process in the controller. The process is removed from the controller.
    pub fn kill_process(&mut self, process: u32) -> Result<(), ProcessError> {
        let process = self
            .processes
            .iter_mut()
            .find(|p| p.id() == process)
            .ok_or(ProcessError::ProcessNotFound)?;

        process.kill()?;

        let id = process.id();
        self.remove_process(id)?;

        Ok(())
    }
    /// Gets the exit code of a process in the controller. If the process has not exited, `None` is returned.
    pub fn get_exit_code(&mut self, process: u32) -> Result<Option<i32>, ProcessError> {
        let process = self
            .processes
            .iter_mut()
            .find(|p| p.id() == process)
            .ok_or(ProcessError::ProcessNotFound)?;

        let id = process.id();

        match process.try_wait()? {
            Some(status) => {
                self.remove_process(id)?;
                Ok(status.code())
            }
            None => Ok(None),
        }
    }
    /// Spawns a new process. The process is added to the controller.
    pub fn spawn_process_raw(
        &mut self,
        binary: &Path,
        args: &[&str],
        env: Option<&[(String, String)]>,
    ) -> Result<&Child, ProcessError> {
        let mut command = std::process::Command::new(binary);
        command.args(args);

        if let Some(env) = env {
            for (key, value) in env {
                command.env(key, value);
            }
        }

        let process = command.spawn()?;
        self.add_process(process);
        Ok(self.processes.last().unwrap())
    }

    pub fn spawn_process(&mut self, process: &Process) -> Result<&Child, ProcessError> {
        let binary = PathBuf::from(process.process.clone());
        let args = process.args.clone();
        let args_ref = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let env = process.env.clone();
        let env_ref = env
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();

        self.spawn_process_raw(&binary, &args_ref, Some(&env_ref))
    }
    /// Waits for a process to exit. Returns the exit code of the process, and removes it from the controller.
    pub fn wait_for_process(&mut self, process: u32) -> Result<Option<i32>, ProcessError> {
        let process = self
            .processes
            .iter_mut()
            .find(|p| p.id() == process)
            .ok_or(ProcessError::ProcessNotFound)?;

        let status = process.wait()?;
        let exit_code = status.code();
        let id = process.id();
        self.remove_process(id)?;
        Ok(exit_code)
    }
}

impl Default for ExecutionController {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("Process not found")]
    ProcessNotFound,
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_process() {
        let mut controller = ExecutionController::new();
        let process = controller
            .spawn_process_raw(Path::new("ls"), &["-l"], None)
            .unwrap();
        let process_id = process.id();
        let code = controller.wait_for_process(process_id).unwrap();
        assert!(controller.processes.is_empty());
        // Can't guarantee the exit code of ls on all systems
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        assert_eq!(code, Some(0));
    }

    #[test]
    fn test_kill_process() {
        let mut controller = ExecutionController::new();
        let process = controller
            .spawn_process_raw(Path::new("sleep"), &["1000"], None)
            .unwrap();
        let process_id = process.id();
        controller.kill_process(process_id).unwrap();
        assert!(controller.processes.is_empty());
    }

    #[test]
    fn test_kill_all() {
        let mut controller = ExecutionController::new();
        controller
            .spawn_process_raw(Path::new("sleep"), &["1000"], None)
            .unwrap();
        controller
            .spawn_process_raw(Path::new("sleep"), &["1000"], None)
            .unwrap();
        controller.kill_all().unwrap();
        assert!(controller.processes.is_empty());
    }
}
