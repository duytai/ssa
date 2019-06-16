use std::io::*;
use std::fs;
use std::process::{ Command, Output };
use std::os::unix::fs::OpenOptionsExt;
use crate::option::{
    SolidityOption,
    SolidityOutput,
    SolidityOutputKind,
    SolidityASTOutput,
};

pub struct Solidity<'a> {
    option: SolidityOption<'a>,
}

impl<'a> Solidity<'a> {
    pub fn new(option: SolidityOption<'a>) -> Self {
        Solidity { option }
    }

    pub fn compile(&self) -> Result<SolidityOutput> {
        let mut version = String::from("v0.4.24");
        let source = fs::read_to_string(self.option.contract)?;
        let source = source.trim();
        if source.starts_with("pragma solidity ^") {
            let temp = source.split("^").collect::<Vec<&str>>();
            let temp = temp[1].split(";").collect::<Vec<&str>>();
            version = format!("v{}", temp[0]);
        }
        let solc_url = "https://github.com/ethereum/solidity/releases/download";
        let solc_url = format!("{}/{}/solc-static-linux", solc_url, version);
        let compiler_path = self.option.bin_dir.join(&version);
        if !compiler_path.exists() {
            let mut response = reqwest::get(&solc_url).unwrap();
            let mut dest = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o777)
                .open(compiler_path)?;
            copy(&mut response, &mut dest)?;
        }
        let compiler_path = self.option.bin_dir.join(&version);
        let Output { stdout, status, .. } = Command::new(compiler_path)
            .arg("--combined-json")
            .arg("ast")
            .arg(self.option.contract)
            .output()?;
        assert!(status.success());
        match self.option.kind {
            SolidityOutputKind::AST => {
                let ast_output = SolidityASTOutput {
                    source: source.to_string(),
                    ast: String::from_utf8(stdout).expect("Invalid utf8 format"),
                };
                Ok(SolidityOutput::AST(ast_output))
            }
        }
    }
}
