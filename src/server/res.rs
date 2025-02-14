use std::{fs, io::Write, path::PathBuf};

use log::{debug, info, warn};

/// A struct to manage resources like files and directories.
#[derive(Debug)]
pub struct ResourceManager {
    pub working_dir: PathBuf,
}

impl ResourceManager {
    /// Creates a new `ResourceManager` with the specified working directory.
    pub fn new(working_dir: PathBuf) -> Self {
        // Ensure the working directory exists
        if !working_dir.exists() {
            fs::create_dir_all(&working_dir).expect("Failed to create working directory");
        }
        let working_dir = working_dir
            .canonicalize()
            .expect("Failed to canonicalize path");
        Self { working_dir }
    }
    /// Returns the working directory.
    pub fn get_working_dir(&self) -> &PathBuf {
        &self.working_dir
    }
    /// Sets the working directory.
    pub fn set_working_dir(&mut self, path: PathBuf) {
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create working directory");
        }
        let working_dir = path.canonicalize().expect("Failed to canonicalize path");
        self.working_dir = working_dir;
    }

    /// Creates a new directory at the specified path.
    pub fn write_file(&self, filename: &str, content: &[u8]) -> std::io::Result<()> {
        // Create the full path to the file
        let file_path = self.join_path_secure(filename)?;
        info!("Writing file: {:?}", file_path);
        if let Err(e) = fs::create_dir_all(&self.working_dir) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }

        // Write the content to the file
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content)?;

        Ok(())
    }

    /// Reads a file from the working directory.
    pub fn read_file(&self, filename: &str) -> std::io::Result<Vec<u8>> {
        let f = self.join_path_secure(filename)?;
        if f.exists() {
            fs::read(f)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", filename),
            ))
        }
    }

    /// Deletes a file or directory from the working directory.
    pub fn delete_file(&self, filename: &str) -> std::io::Result<()> {
        let f = self.join_path_secure(filename)?;
        if f.exists() {
            if f.is_dir() {
                fs::remove_dir_all(f)
            } else {
                fs::remove_file(f)
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", filename),
            ))
        }
    }

    pub fn reset(&mut self) -> std::io::Result<()> {
        // Remove all files and directories in the working directory
        if self.working_dir.exists() {
            fs::remove_dir_all(&self.working_dir)?;
        }
        // Recreate the working directory
        fs::create_dir_all(&self.working_dir)?;
        Ok(())
    }

    /// Joins the working directory with the given path and checks for path traversal.
    fn join_path_secure(&self, path: &str) -> std::io::Result<PathBuf> {
        let full_path = self.working_dir.join(path);
        debug!("Full path: {:?}", full_path);
        if full_path.exists() {
            debug!("Path exists: {:?}", full_path);
        } else {
            // Fallback to failing if .. sequence is detected
            debug!("Path does not exist: {:?}", full_path);
            if path.contains("..") {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Path traversal detected",
                ));
            }
            return Ok(full_path);
        }
        let cannon = full_path.canonicalize()?;
        if cannon.starts_with(&self.working_dir) {
            debug!("Resolved path: {:?}", cannon);
            Ok(cannon)
        } else {
            warn!("Path traversal detected: {:?}", cannon);
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path traversal detected",
            ))
        }
    }

    // TODO: Allow .tar.gz files to be extracted
}

impl Default for ResourceManager {
    fn default() -> Self {
        let working_dir = PathBuf::from("remote_ldr_working_dir");
        Self::new(working_dir)
    }
}
