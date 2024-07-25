use anyhow::Result;
use std::path::PathBuf;

pub struct DataBase {
    pub host: String,
    pub path: PathBuf,
    pub table: Vec<Table>,
}

pub struct Table {
    pub name: String,
    pub path: String,
    pub rank: f64,
    pub last_accessed: u64,
}

impl DataBase {
    pub fn get_connect() -> Result<()> {
        // 打开文件目录
        // 根据Host创建的文件内容
        // read 出来
        Ok(())
    }

    pub fn create_db() -> Result<()> {
        todo!()
    }

    pub fn update_db() -> Result<()> {
        todo!()
    }
}
