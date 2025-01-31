use std::process::{Command, Output};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("Docker command failed: {0}")]
    CommandFailed(String),
    #[error("Failed to execute docker command: {0}")]
    ExecutionFailed(#[from] std::io::Error),
}

pub struct DockerCompose {
    compose_file: String,
}

impl DockerCompose {
    pub fn new(compose_file: &str) -> Self {
        Self {
            compose_file: compose_file.to_string(),
        }
    }

    fn run_command(&self, args: &[&str]) -> Result<Output, DockerError> {
        let output = Command::new("docker")
            .current_dir("clients")
            .args(["compose", "-f", &self.compose_file])
            .args(args)
            .output()?;

        if !output.status.success() {
            let error_message = format!(
                "{} | stdout: {}",
                String::from_utf8_lossy(&output.stderr).trim(),
                String::from_utf8_lossy(&output.stdout).trim()
            );
            return Err(DockerError::CommandFailed(error_message));
        }

        Ok(output)
    }

    fn get_project_name(&self) -> String {
        let file_name = Path::new(&self.compose_file)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("client");

        format!("odometer-{}", file_name)
    }


    pub fn up(&self) -> Result<(), DockerError> {
        self.run_command(&["-p", &self.get_project_name(),"up","-d"])?;
        Ok(())
    }

    pub fn down(&self) -> Result<(), DockerError> {
        self.run_command(&["down", "--volumes"])?;
        Ok(())
    }
}