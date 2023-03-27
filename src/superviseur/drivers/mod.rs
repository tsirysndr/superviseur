pub mod exec;
pub mod flox;

use anyhow::Error;

pub trait DriverPlugin {
    fn start(&self) -> Result<(), Error>;
    fn stop(&self) -> Result<(), Error>;
    fn restart(&self) -> Result<(), Error>;
    fn status(&self) -> Result<(), Error>;
    fn logs(&self) -> Result<(), Error>;
    fn exec(&self) -> Result<(), Error>;
}
