pub mod devbox;
pub mod devenv;
pub mod docker;
pub mod exec;
pub mod flox;

use std::thread;

use anyhow::Error;
use async_trait::async_trait;
use dyn_clone::DynClone;

use crate::types::configuration::ConfigurationData;

use self::docker::setup::setup_docker;

#[async_trait]
pub trait DriverPlugin: DynClone {
    async fn start(&self, project: String) -> Result<(), Error>;
    async fn stop(&self, project: String) -> Result<(), Error>;
    async fn restart(&self, project: String) -> Result<(), Error>;
    async fn status(&self) -> Result<(), Error>;
    async fn logs(&self) -> Result<(), Error>;
    async fn exec(&self) -> Result<(), Error>;
    async fn build(&self, project: String) -> Result<(), Error>;
}

pub fn setup_drivers(cfg: ConfigurationData) {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            setup_docker(&cfg).await?;
            Ok::<(), Error>(())
        })
    });
}
