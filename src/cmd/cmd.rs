use super::{add::Add, search::Search, Process};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
pub enum Cmd {
    Add(Add),
    Edit,
    Search(Search),
}
