pub mod exec;
pub mod flox;

use anyhow::Error;
use dyn_clone::DynClone;

pub trait DriverPlugin: DynClone {
    fn start(&self, project: String) -> Result<(), Error>;
    fn stop(&self, project: String) -> Result<(), Error>;
    fn restart(&self, project: String) -> Result<(), Error>;
    fn status(&self) -> Result<(), Error>;
    fn logs(&self) -> Result<(), Error>;
    fn exec(&self) -> Result<(), Error>;
    fn build(&self, project: String) -> Result<(), Error>;
}
