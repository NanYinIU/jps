pub mod add;
pub mod cmd;
pub mod search;
pub use crate::cmd::cmd::*;
use anyhow::Result;

pub trait Process {
    fn process(&self) -> Result<()>;
}
impl Process for Cmd {
    fn process(&self) -> Result<()> {
        match self {
            Cmd::Add(cmd) => cmd.process(),
            Cmd::Search(cmd) => cmd.process(),
            _ => todo!(),
        }
    }
}
