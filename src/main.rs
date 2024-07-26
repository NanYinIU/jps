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
#[cfg(test)]
pub mod test {
    use serde::Serialize;

    use crate::db::{DataBase, Table};

    #[test]
    pub fn test_bincode() {
        // The object that we will serialize.
        let target: Option<String> = Some("hello world".to_string());

        let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
        println!("{:?}", encoded);
        let decoded: Option<String> = bincode::deserialize(&encoded[..]).unwrap();
        println!("{:?}", decoded);
        assert_eq!(target, decoded);
    }
    #[test]
    pub fn db_serialize() {
        let t = Table::new(String::from("t1"), String::from("path"), 1.0, 2).unwrap();
        let tables = vec![t];
        let db: DataBase = DataBase::create_db(String::from("ver"), tables).unwrap();
        let dbs = vec![db];
        let encoded = DataBase::serialize(&dbs).unwrap();
        println!("{:?}", encoded);
        let decoded = DataBase::deserialize(&encoded).unwrap();
        println!("{:?}", decoded);
    }
}
