//! Docker client abstraction layer.
//!
//! Provides a [`DockerClient`] trait for container operations and a concrete
//! [`CliDockerClient`] implementation using the Docker CLI.

pub mod client;

pub use client::{CliDockerClient, ContainerInfo, DockerClient};

#[cfg(feature = "testing")]
mod test_mock {
    use super::*;
    use anyhow::Result;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;
    use std::sync::{Mutex, RwLock};

    /// Thread-safe mock for DockerClient supporting configurable behavior.
    pub struct StubDockerClient {
        build_image_result: Mutex<Result<(), String>>,
        run_interactive_result: Mutex<Result<ExitStatus, String>>,
        run_detached_result: Mutex<Result<(), String>>,
        stop_containers_result: Mutex<Result<(), String>>,
        list_containers_result: Mutex<Result<Vec<ContainerInfo>, String>>,
        get_logs_result: Mutex<Result<String, String>>,
        captured_calls: RwLock<Vec<String>>,
    }

    impl StubDockerClient {
        pub fn new() -> Self {
            // Create a successful ExitStatus using platform-specific tricks
            let success_exit = std::process::Command::new("true")
                .output()
                .map(|o| o.status)
                .unwrap_or_else(|_| ExitStatus::from_raw(0));

            Self {
                build_image_result: Mutex::new(Ok(())),
                run_interactive_result: Mutex::new(Ok(success_exit)),
                run_detached_result: Mutex::new(Ok(())),
                stop_containers_result: Mutex::new(Ok(())),
                list_containers_result: Mutex::new(Ok(vec![])),
                get_logs_result: Mutex::new(Ok(String::new())),
                captured_calls: RwLock::new(Vec::new()),
            }
        }

        pub fn with_build_error(self, err: impl Into<String>) -> Self {
            *self.build_image_result.lock().unwrap() = Err(err.into());
            self
        }

        pub fn with_run_interactive_error(self, err: impl Into<String>) -> Self {
            *self.run_interactive_result.lock().unwrap() = Err(err.into());
            self
        }

        pub fn with_run_detached_error(self, err: impl Into<String>) -> Self {
            *self.run_detached_result.lock().unwrap() = Err(err.into());
            self
        }

        pub fn with_stop_containers_error(self, err: impl Into<String>) -> Self {
            *self.stop_containers_result.lock().unwrap() = Err(err.into());
            self
        }

        pub fn with_list_containers(self, containers: Vec<ContainerInfo>) -> Self {
            *self.list_containers_result.lock().unwrap() = Ok(containers);
            self
        }

        pub fn with_get_logs(self, logs: impl Into<String>) -> Self {
            *self.get_logs_result.lock().unwrap() = Ok(logs.into());
            self
        }

        pub fn get_calls(&self) -> Vec<String> {
            self.captured_calls.read().unwrap().clone()
        }
    }

    impl Default for StubDockerClient {
        fn default() -> Self {
            Self::new()
        }
    }

    impl DockerClient for StubDockerClient {
        fn build_image(
            &self,
            image_name: &str,
            dockerfile_path: &str,
            build_context: &str,
            build_args: &[(&str, &str)],
        ) -> Result<()> {
            self.captured_calls.write().unwrap().push(format!(
                "build_image({}, {}, {}, {:?})",
                image_name, dockerfile_path, build_context, build_args
            ));
            self.build_image_result
                .lock()
                .unwrap()
                .clone()
                .map_err(anyhow::Error::msg)
        }

        fn run_interactive(
            &self,
            image: &str,
            volume_mounts: &[(&str, &str)],
            workdir: &str,
            cmd: &[&str],
        ) -> Result<ExitStatus> {
            self.captured_calls.write().unwrap().push(format!(
                "run_interactive({}, {:?}, {}, {:?})",
                image, volume_mounts, workdir, cmd
            ));
            self.run_interactive_result
                .lock()
                .unwrap()
                .clone()
                .map_err(anyhow::Error::msg)
        }

        fn run_detached(
            &self,
            image: &str,
            name: Option<&str>,
            ports: &[(&str, &str)],
            volume_mounts: &[(&str, &str)],
            workdir: &str,
            cmd: &[&str],
        ) -> Result<()> {
            self.captured_calls.write().unwrap().push(format!(
                "run_detached({}, {:?}, {:?}, {:?}, {}, {:?})",
                image, name, ports, volume_mounts, workdir, cmd
            ));
            self.run_detached_result
                .lock()
                .unwrap()
                .clone()
                .map_err(anyhow::Error::msg)
        }

        fn stop_containers(&self, ids: &[&str]) -> Result<()> {
            self.captured_calls
                .write()
                .unwrap()
                .push(format!("stop_containers({:?})", ids));
            self.stop_containers_result
                .lock()
                .unwrap()
                .clone()
                .map_err(anyhow::Error::msg)
        }

        fn list_containers(&self, image_filter: &str) -> Result<Vec<ContainerInfo>> {
            self.captured_calls
                .write()
                .unwrap()
                .push(format!("list_containers({})", image_filter));
            self.list_containers_result
                .lock()
                .unwrap()
                .clone()
                .map_err(anyhow::Error::msg)
        }

        fn get_logs(&self, container_name: &str) -> Result<String> {
            self.captured_calls
                .write()
                .unwrap()
                .push(format!("get_logs({})", container_name));
            self.get_logs_result
                .lock()
                .unwrap()
                .clone()
                .map_err(anyhow::Error::msg)
        }
    }
}

#[cfg(feature = "testing")]
pub use test_mock::StubDockerClient;
