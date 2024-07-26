use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fmt::format,
    fs::{self, File},
    io,
    path::PathBuf,
};

use crate::test::db_serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBase {
    pub host: String,
    pub tables: Vec<Table>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub name: String,
    pub path: String,
    pub rank: f64,
    pub last_accessed: u64,
}

impl DataBase {
    pub fn get_connect(self) -> Result<()> {
        // 打开文件目录
        let connect_dir = "../";
        let pb = PathBuf::from(connect_dir);
        pb.join(self.host).with_extension("txt");
        // 根据Host创建的文件内容
        match std::fs::read(pb) {
            Ok(bytes) => {
                todo!()
            }
            Err(e) => return Err(anyhow!("dbError:{}", e)),
        }
        // read 出来
        Ok(())
    }

    pub fn create_db(&mut self, host: String, tables: Vec<Table>) -> Result<()> {
        let db: DataBase = DataBase {
            host: host.clone(),
            tables,
        };
        let content = Self::serialize(&vec![db]).unwrap();
        let path = PathBuf::from("dir").join(host).with_extension("txt");
        // 需要确保这个文件不存在
        let after_create = match fs::read(&path) {
            Ok(_) => todo!("do add table content"),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    fs::write(&path, content).with_context(|| format!("error!!"))?
                }
            }
        };
        // fs::write(
        //     PathBuf::from("dir").join(host).with_extension("txt"),
        //     &content,
        // )?;
        Ok(())
    }

    pub fn update_db(&mut self, host: String) -> Result<()> {
        self.host = host;
        Ok(())
    }

    pub fn serialize(dbs: &Vec<DataBase>) -> Result<Vec<u8>> {
        (|| -> bincode::Result<_> {
            let magic_number = "JPS";
            let magic_number_size = bincode::serialized_size(magic_number)?;
            let content_size = bincode::serialized_size(&dbs)?;
            let buffer_size = magic_number_size + content_size;
            let mut buffer = Vec::with_capacity(buffer_size as usize);
            // 这里的version
            bincode::serialize_into(&mut buffer, magic_number)?;
            bincode::serialize_into(&mut buffer, dbs)?;
            Ok(buffer)
        })()
        .context("error")
    }

    pub fn deserialize(buffer: &Vec<u8>) -> Result<Vec<DataBase>> {
        let magic_number = "JPS";
        let magic_number_size = bincode::serialized_size(magic_number)?;
        if buffer.len() < magic_number_size as usize {
            return Err(anyhow!("error"));
        }
        let (magic_number, content) = buffer.split_at(magic_number_size as usize);
        // 分别反序列化
        let db: Vec<DataBase> = match bincode::deserialize(&magic_number).unwrap() {
            "JPS" => bincode::deserialize(&content)?,
            _ => return Err(anyhow!("error version code")),
        };
        Ok(db)
    }
}

impl Table {
    pub fn new(name: String, path: String, rank: f64, last_accessed: u64) -> Result<Table> {
        let t = Table {
            name,
            path,
            rank,
            last_accessed,
        };
        Ok(t)
    }
}
