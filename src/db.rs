use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf, sync::Arc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataBase {
    pub host: String,
    #[serde(with = "arc_vec")]
    pub tables: Vec<Arc<Table>>,
    pub path: PathBuf,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub name: String,
    pub path: String,
    pub rank: f64,
    pub last_accessed: u64,
}

impl DataBase {
    fn connect(db: &DataBase) -> Result<DataBase> {
        let path = &db.path;
        // 需要确保这个文件不存在
        return match fs::read(&path) {
            Ok(buffer) => {
                let mut this_db = Self::deserialize(&buffer)?;
                let tables = &db.tables;
                if !(tables.is_empty()) {
                    this_db.tables.extend(db.tables.iter().cloned())
                }
                return Ok(this_db);
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    let content = Self::serialize(&db).unwrap();
                    fs::write(&path, content)
                        .with_context(|| format!("create host file error!!"))?;
                    return Ok(db.to_owned());
                }
                Err(anyhow!("create db unexpect error !"))
            }
        };
    }

    fn build_path_buf(host: &str) -> PathBuf {
        PathBuf::from("dir").join(host).with_extension("txt")
    }

    pub fn create_or_update(&mut self, host: String, tables: Vec<Table>) -> Result<DataBase> {
        let path = Self::build_path_buf(&host);
        let db = DataBase {
            host,
            tables: tables.into_iter().map(|x| Arc::new(x)).collect(),
            path,
        };
        Self::connect(&db)
    }

    pub fn read(&mut self, host: String) -> Result<DataBase> {
        let path = Self::build_path_buf(&host);
        let db = DataBase {
            host,
            tables: vec![],
            path,
        };
        Self::connect(&db)
    }

    fn serialize(db: &DataBase) -> Result<Vec<u8>> {
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

    fn deserialize(buffer: &Vec<u8>) -> Result<DataBase> {
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

mod arc_vec {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(data: &Vec<Arc<Table>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vec: Vec<&Table> = data.iter().map(AsRef::as_ref).collect();
        vec.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Arc<Table>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<Table> = Vec::deserialize(deserializer)?;
        Ok(vec.into_iter().map(Arc::new).collect())
    }
}
