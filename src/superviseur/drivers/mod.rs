pub mod devbox;
pub mod devenv;
pub mod docker;
pub mod exec;
pub mod flox;

use anyhow::Error;
use async_trait::async_trait;
use dyn_clone::DynClone;

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
