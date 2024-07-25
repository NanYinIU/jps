use clap::Parser;

use crate::Process;
use anyhow::Result;

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Add {
    pub host: String,
    pub name: String,
    pub order: Vec<String>,
}
impl Process for Add {
    fn process(&self) -> Result<()> {
        println!("{:?}", &self);
        // 这里需要处理下数据的使用方式
        //
        todo!()
    }
}
