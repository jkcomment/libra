mod reboot;
mod stop_container;

use failure;
pub use reboot::Reboot;
use std::fmt::Display;
pub use stop_container::StopContainer;

pub trait Action: Display + Sync {
    fn apply(&self) -> failure::Result<()>;
    fn is_complete(&self) -> bool;
}

pub trait Effect: Display + Sync {
    fn activate(&self) -> failure::Result<()>;
    fn deactivate(&self) -> failure::Result<()>;
}
