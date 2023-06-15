use std::{fs, io};
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;

use calamine::{DataType, open_workbook, Range, Reader, Xlsx};

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
    /// 加载`path`目录下的所有excel工作表或者`path`指定的工作表文件，并通过`callback`依次消费。
    ///
    /// path可以是目录或excel文件路径
    fn load<Func>(&self, args: &BuildArgs, path: &str, callback: Func) where Func: Fn(&ConfigTable) {
        let p = Path::new(args.path.as_str());

        let xlsx_list: Vec<Option<(String, Xlsx<_>)>>;
        if p.is_dir() {
            xlsx_list = match fs::read_dir(path) {
                Ok(entries) => {
                    entries.map(|entry| {
                        match entry.map(|e| {
                            e.file_name().to_string_lossy().to_string()
                        }).and_then(|filename| {
                            load_xlsx(path, filename.as_str()).ok_or(io::Error::from(ErrorKind::NotFound))
                        }) {
                            Ok(wb) => { Some(wb) }
                            Err(err) => {
                                eprintln!("Error load xlsx file {}", err);
                                None
                            }
                        }
                    }).collect()
                }
                Err(err) => {
                    eprintln!("Error read dir {}.{}", p.to_string_lossy(), err);
                    vec![]
                }
            };
        } else {
            let wb = load_xlsx(".", path);
            xlsx_list = vec![wb];
        }
        xlsx_list.into_iter()
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .flat_map(|t| load_config_table(args, t.0.as_str(), t.1))
            .for_each(|t| callback(&t));
    }
}

/// 读取[dir]目录下的[filename]文件.
///
/// 文件需要是`xlsx`或者`xls`格式，忽略`~$`开头的临时文件。
///
/// 返回文件名和文件组成的元组[Option]
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

/// 转换excel文件为[ConfigTable]，文件内每个`sheet`将对应一个[ConfigTable]。
///
fn load_config_table(args: &BuildArgs, filename: &str, mut workbook: Xlsx<BufReader<File>>) -> Vec<ConfigTable> {
    let sheets = workbook.worksheets();
    match args.load {
        LoadMode::AllSheets => sheets.into_iter()
            .map(|s| build_table(&args, filename, s)).collect(),
        LoadMode::FirstSheet => vec![build_table(&args, filename, sheets[0].to_owned())]
    }
}

/// 转换一个excel文件工作簿(sheet)为[ConfigTable]
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