use std::{fs, io};
use std::arch::x86_64::_mm256_undefined_pd;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use std::path::Path;

use calamine::{DataType, open_workbook, Range, Reader, Xlsx, XlsxError};

use crate::args::{BuildArgs, LoadMode};
use crate::context::func::ConfigLoader;
use crate::context::model::{ConfigHeaderBuilder, ConfigTable, ConfigTableBuilder};

pub struct XlsxConfigLoader {}

impl XlsxConfigLoader {
    pub fn new() -> XlsxConfigLoader {
        XlsxConfigLoader {}
    }
}

impl ConfigLoader for XlsxConfigLoader {
    fn load<Func>(&self, args: &BuildArgs, path: &str, callback: Func) where Func: Fn(&ConfigTable) {
        let p = Path::new(args.path.as_str());

        let xlsx_list: Vec<Option<(String, Xlsx<_>)>>;
        if p.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                xlsx_list = entries.map(|entry| {
                    let wb = match entry.map(|e| {
                        e.file_name().to_string_lossy().to_string()
                    }).and_then(|filename| {
                        load_xlsx(path, filename.as_str()).ok_or(io::Error::from(ErrorKind::NotFound))
                    }) {
                        Ok(wb) => { Some(wb) }
                        Err(err) => {
                            eprintln!("Error load xlsx file {}", err);
                            None
                        }
                    };
                    wb
                }).collect();
            } else {
                xlsx_list = vec![]
            }
        } else {
            let wb = load_xlsx(".", path);
            xlsx_list = vec![wb];
        }
        xlsx_list.into_iter()
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            // TODO flat_map sheet
            // .map(|t| {
            //     t.1.sheet_names().to_vec().iter().for_each(|s| {
            //         println!("sheet: {}", s);
            //     });
            //
            //     // let mut string = t.1.sheet_names().clone();
            //     let table = ConfigTableBuilder::new()
            //         // TODO filling fields
            //         .set_name(t.0)
            //         // .set_sheet_name(string[0].clone())
            //         .build();
            //     Box::new(table)
            // })
            .flat_map(|t| load_config_table(args, t.0.as_str(), t.1))
            .for_each(|t| callback(&t));
    }
}

fn load_xlsx(dir: &str, filename: &str) -> Option<(String, Xlsx<BufReader<File>>)> {
    if filename.starts_with("~$") {
        return None;
    }
    if !filename.ends_with(".xlsx") && !filename.ends_with(".xls") {
        return None;
    }
    println!("loading xlsx: {},{}", dir, filename);
    let mut slash = "";
    if !dir.ends_with("/") {
        slash = "/"
    }
    let filepath = format!("{}{}{}", dir, slash, filename);
    match open_workbook(filepath) {
        Ok(wb) => Some((String::from(filename), wb)),
        Err(err) => {
            eprintln!("Error open file {}", err);
            None
        }
    }
}

fn load_config_table(args: &BuildArgs, filename: &str, mut workbook: Xlsx<BufReader<File>>) -> Vec<ConfigTable> {
    let sheets = workbook.worksheets();
    match args.load {
        LoadMode::AllSheets => sheets.into_iter()
            .map(|s| build_table(&args, filename, s)).collect(),
        LoadMode::FirstSheet => vec![build_table(&args, filename, sheets[0].to_owned())]
    }
}

fn build_table(args: &BuildArgs, filename: &str, sheet_tuple: (String, Range<DataType>)) -> ConfigTable {
    let mut builder = ConfigTableBuilder::new().set_sheet_name(sheet_tuple.0).set_name(String::from(filename));
    let mut index = 0;
    let mut comments = vec![];
    let mut server_types = vec![];
    let mut server_names = vec![];
    let mut server_flags = vec![];
    for row in sheet_tuple.1.rows() {
        println!("row={:?}, row[0]={:?}", row, row[0]);
        if index == args.comment_pattern.line_no {
            comments = row.to_vec();
        }
        if index == args.mode_pattern.line_no {
            server_flags = row.to_vec();
        }
        if index == args.name_pattern.line_no {
            server_names = row.to_vec();
        }
        if index == args.type_pattern.line_no {
            server_types = row.to_vec();
        }
        builder = builder.add_data(row.to_vec());
        index += 1;
    }
    for (index, f) in server_flags.iter().enumerate() {
        let s = f.as_string().unwrap();
        let fmt_flag = s.trim();
        let f = (args.mode_pattern.extractor)(fmt_flag);
        if f.eq(&args.mode.to_string()) || f.eq("all") {
            let hb = ConfigHeaderBuilder::new()
                .set_field_name(server_names[index].as_string().unwrap())
                .set_field_type(server_types[index].as_string().unwrap())
                .set_comment(comments[index].as_string().unwrap());
            builder = builder.add_header(hb);
        }
    }
    builder.build()
}