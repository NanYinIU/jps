use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fmt::format,
    fs::{self, File},
    io,
    path::PathBuf,
};

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

    pub fn create_db(&mut self, host: String, tables: Vec<Table>) -> Result<DataBase> {
        let path = PathBuf::from("dir")
            .join(host.clone())
            .with_extension("txt");
        // 需要确保这个文件不存在
        return match fs::read(&path) {
            Ok(buffer) => {
                let mut this_db = Self::deserialize(&buffer)?;
                for i in tables {
                    this_db.tables.push(i)
                }
                return Ok(this_db);
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    let db: DataBase = DataBase { host, tables };
                    let content = Self::serialize(&db).unwrap();
                    fs::write(&path, content).with_context(|| format!("error!!"))?;
                    return Ok(db);
                }
                Err(anyhow!("unexpect error !"))
            }
        };
    }

    pub fn update_db(&mut self, host: String) -> Result<()> {
        self.host = host;
        Ok(())
    }

    pub fn serialize(db: &DataBase) -> Result<Vec<u8>> {
        (|| -> bincode::Result<_> {
            let magic_number = "JPS";
            let magic_number_size = bincode::serialized_size(magic_number)?;
            let content_size = bincode::serialized_size(&db)?;
            let buffer_size = magic_number_size + content_size;
            let mut buffer = Vec::with_capacity(buffer_size as usize);
            // 这里的version
            bincode::serialize_into(&mut buffer, magic_number)?;
            bincode::serialize_into(&mut buffer, db)?;
            Ok(buffer)
        })()
        .context("error")
    }

    pub fn deserialize(buffer: &Vec<u8>) -> Result<DataBase> {
        let magic_number = "JPS";
        let magic_number_size = bincode::serialized_size(magic_number)?;
        if buffer.len() < magic_number_size as usize {
            return Err(anyhow!("error"));
        }
        let (magic_number, content) = buffer.split_at(magic_number_size as usize);
        // 分别反序列化
        let db: DataBase = match bincode::deserialize(&magic_number).unwrap() {
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
