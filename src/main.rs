use clap::Parser;
pub use std::process::ExitCode;
pub mod cmd;
pub mod db;

pub use crate::cmd::{Cmd, Process};

pub fn main() -> ExitCode {
    match Cmd::parse().process() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::FAILURE,
    }
}
