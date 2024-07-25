use std::process::{Command, Stdio};

use clap::Parser;

use crate::Process;
use anyhow::Result;

#[derive(Debug, Parser)]
#[clap(author)]
pub struct Search {
    pub host: String,
    pub name: String,
    pub order: Vec<String>,
}

impl Process for Search {
    fn process(&self) -> Result<()> {
        println!("{:?}", &self);
        let host = &self.host;
        let orders = &self.order;
        let mut expect_script = String::from(format!(
            r#"
                spawn ssh {host}
            "#,
        ));
        for ele in orders {
            expect_script.push_str(
                format!(
                    r#"expect "$ "
                        send "{ele}\r"
                        "#
                )
                .as_str(),
            )
        }
        expect_script.push_str(r#"interact"#);
        println!("expect script is {expect_script}");

        let mut child = Command::new("expect")
            .arg("-c")
            .arg(expect_script)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        println!("Expect command is running. Press Ctrl+C to exit.");

        // 等待子进程结束
        child.wait()?;

        println!("Expect command has terminated.");
        Ok(())
    }
}
