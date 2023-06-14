use std::{fs, io};
use std::arch::x86_64::_mm256_undefined_pd;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use std::path::Path;

use calamine::{open_workbook, Reader, Xlsx};

use crate::args::BuildArgs;
use crate::context::func::ConfigLoader;
use crate::context::model::{ConfigTable, ConfigTableBuilder};

pub struct XlsxConfigLoader {}

impl XlsxConfigLoader {
    pub fn new() -> XlsxConfigLoader {
        XlsxConfigLoader {}
    }
}

impl ConfigLoader for XlsxConfigLoader {
    fn load(&self, args: &BuildArgs, path: &str, callback: fn(&ConfigTable)) {
        let p = Path::new(args.path.as_str());

        let xlsxes: Vec<Xlsx<_>>;
        if p.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                // let ve = entries.map(|entry| {
                //     let wb = entry.map(|e| {
                //         e.file_name().to_string_lossy().to_string()
                //     }).and_then(|filename| {
                //         load_xlsx(path, filename.as_str()).ok_or(io::Error::from(ErrorKind::NotFound))
                //     });
                //     wb
                // }).flat_map(|wb|{
                //     let workbook = wb.unwrap();
                //     (workbook.workbook.worksheets())
                // }).for_each(|tuple|{
                // });
                xlsxes = entries.map(|entry| {
                    let wb = match entry.map(|e| {
                        e.file_name().to_string_lossy().to_string()
                    }).and_then(|filename| {
                        load_xlsx(path, filename.as_str()).ok_or(io::Error::from(ErrorKind::NotFound))
                    }) {
                        Ok(wb) => { wb }
                        Err(err) => { eprintln!("Error open file {}", err) }
                    };
                    wb
                }).collect();
                // for entry in entries {
                //     let wb = entry.map(|e| {
                //         e.file_name().to_string_lossy().to_string()
                //     }).and_then(|filename| {
                //         load_xlsx(path, filename.as_str()).ok_or(io::Error::from(ErrorKind::NotFound))
                //     });
                // }
            }
        } else {
            let wb = match load_xlsx(".", path) {
                Some(wb) => { wb }
                None => {}
            };
            xlsxes = vec![wb];
        }
        xlsxes.iter()
            // TODO flat_map sheet
            .map(|t| {
                ConfigTableBuilder::new()
                    // TODO filling fields
                    .build()
            }).for_each(|t| {
            callback(&t)
        });
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
    let mut slash = "";
    if !dir.ends_with("/") {
        slash = "/"
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

fn load_config_table(args: &BuildArgs, workbook: Xlsx<BufReader<File>>) -> Vec<ConfigTable> {
    vec![]
}