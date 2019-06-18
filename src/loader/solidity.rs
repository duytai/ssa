use std::io::*;
use std::fs;
use std::process::{ Command, Output };
use std::collections::HashMap;
use std::os::unix::fs::OpenOptionsExt;
use crate::loader::option::{
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

    fn extract_source_map(&self, ast: &String) -> Result<HashMap<String, String>> {
        let mut ret = HashMap::new();
        let source_list = ast.split("\"sourceList\":[").collect::<Vec<&str>>();
        let source_list = source_list[1];
        if let Some(close_bracket) = source_list.find("]") {
            let source_list = &source_list[0..close_bracket];
            let source_list = source_list.split("\"").collect::<Vec<&str>>();
            for i in 0..source_list.len() {
                if i % 2 == 1 {
                    let path = source_list[i].to_string();
                    let source = fs::read_to_string(&path)?;
                    ret.insert(path, source);
                }
            }
        }
        Ok(ret)
    }

    fn extract_version(&self) -> Result<String> {
        let mut version = String::from("v0.4.24");
        let source = fs::read_to_string(self.option.contract)?;
        let source = source.trim();
        if source.starts_with("pragma solidity ^") {
            let temp = source.split("^").collect::<Vec<&str>>();
            let temp = temp[1].split(";").collect::<Vec<&str>>();
            version = format!("v{}", temp[0]);
        }
        Ok(version)
    }

    pub fn compile(&self) -> Result<SolidityOutput> {
        let version = self.extract_version()?;
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
        let Output { stdout, status, stderr } = Command::new(compiler_path)
            .arg("--combined-json")
            .arg("ast")
            .arg(self.option.contract)
            .output()?;
        assert!(status.success(), String::from_utf8(stderr).unwrap());
        match self.option.kind {
            SolidityOutputKind::AST => {
                let ast = String::from_utf8(stdout).expect("Invalid utf8 format");
                let sources = self.extract_source_map(&ast)?; 
                let ast_output = SolidityASTOutput {
                    sources,
                    ast,
                };
                Ok(SolidityOutput::AST(ast_output))
            }
        }
    }
}
