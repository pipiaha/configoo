use std::{fs, io};
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;

use calamine::{open_workbook, Xlsx};
use crate::args::BuildArgs;

use crate::context::func::ConfigLoader;
use crate::context::model::ConfigTable;

pub struct XlsxConfigLoader {}

impl XlsxConfigLoader {
    pub fn new() -> XlsxConfigLoader {
        XlsxConfigLoader {}
    }
}

impl ConfigLoader for XlsxConfigLoader {
    fn load(&self, args: &BuildArgs, path: &str, callback: fn(&ConfigTable)) {
        let p = Path::new(args.path.as_str());
        if p.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    let wb = entry.map(|e| {
                        e.file_name().to_string_lossy().to_string()
                    }).and_then(|filename| {
                        load_xlsx(path, filename.as_str()).ok_or(io::Error::from(ErrorKind::NotFound))
                    });
                }
            }
        } else {
            let wb = load_xlsx(".", path);
        }
    }
}

fn load_xlsx(dir: &str, filename: &str) -> Option<Xlsx<BufReader<File>>> {
    if filename.starts_with("~$") {
        return None;
    }
    if !filename.ends_with(".xlsx") && !filename.ends_with(".xls") {
        return None;
    }
    println!("loading xlsx: {}", filename);
    let mut slash = "/";
    if dir.ends_with("/") {
        slash = ""
    }
    let filepath = format!("{}{}{}", dir, slash, filename);
    match open_workbook(filepath) {
        Ok(wb) => Some(wb),
        Err(err) => {
            eprintln!("Error open file {}", err);
            None
        }
    }
}

fn load_config_table(args: &BuildArgs, workbook: Xlsx<BufReader<File>>) -> Vec<ConfigTable>{

    vec![]
}