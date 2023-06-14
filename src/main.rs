// use office::Excel;

use std::{env, fs, io};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};

// use crate::args::BuildArgs;
use crate::context::func::{ConfigExporter, ConfigLoader};
use crate::context::model::{ConfigHeaderBuilder, ConfigTable, ConfigTableBuilder};
use crate::export::config::CsvExporter;
use crate::import::importer::XlsxConfigLoader;

mod context;
mod export;
mod import;
// mod args;
mod lang;
mod args;

fn main() {
    // let args = env::args();
    println!("Hello, world!");
    // let mut excel = Excel::open("config/file.xlsx").unwrap();
    // let r = excel.worksheet_range("Sheet1").unwrap();
    // for row in r.rows() {
    //     println!("row={:?}, row[0]={:?}", row, row[0]);
    // }
    let dir = "config";
    //
    // let args = BuildArgs {};

    let loader = XlsxConfigLoader::new();
    loader.load(, dir, |tb| {});

    let path = format!("{}/{}", dir, "RhythmMasterConfig.xlsx");
    let file_path = Path::new(path.as_str());
    // 打开 XLSX 文件
    let mut workbook: Xlsx<_> = match open_workbook(file_path) {
        Ok(wb) => wb,
        Err(err) => {
            eprintln!("can not open file {}", err);
            return;
        }
    };
    // .expect("can not open file");
    // 获取第一个工作表
    let sheet = match workbook.worksheet_range_at(0) {
        Some(Ok(sheet)) => sheet,
        Some(Err(e)) => {
            eprintln!("Failed to open worksheet: {}", e);
            return;
        }
        None => {
            eprintln!("No worksheet found in the workbook");
            return;
        }
    };

    let comment_row_index = 0;
    let server_type_row_index = 2;
    let client_type_row_index = 2;
    let server_name_row_index = 1;
    let client_name_row_index = 1;
    let server_flag_index = 3;
    let client_flag_index = 3;

    let mut comments = vec![];
    let mut server_types = vec![];
    let mut client_types;
    let mut server_names = vec![];
    let mut client_names;
    let mut server_flags = vec![];
    let mut client_flags;

    let mut data = vec![];

    let mut index = 0;
    for row in sheet.rows() {
        println!("row={:?}, row[0]={:?}", row, row[0]);
        if index == comment_row_index {
            comments = row.to_vec();
        }
        if index == server_type_row_index {
            server_types = row.to_vec();
        }
        if index == client_type_row_index {
            client_types = row.to_vec();
        }
        if index == server_name_row_index {
            server_names = row.to_vec();
        }
        if index == client_name_row_index {
            client_names = row.to_vec();
        }
        if index == server_flag_index {
            server_flags = row.to_vec();
        }
        if index == client_flag_index {
            client_flags = row.to_vec();
        }
        data.push(row.to_vec());
        index += 1;
    }

    //  export
    let c = CsvExporter::new();
    c.export_with_default_dir(&ConfigTable {
        name: "test.csv".to_string(),
        data,
        header: vec![],
    });

    // server side code
    let mut tb_builder = ConfigTableBuilder::new();
    for (index, f) in server_flags.iter().enumerate() {
        let s = f.as_string().unwrap();
        let fmt_flag = s.trim();
        if fmt_flag == "server" || fmt_flag == "all" {
            let hb = ConfigHeaderBuilder::new()
                .set_field_name(server_names[index].as_string().unwrap())
                .set_field_type(server_types[index].as_string().unwrap())
                .set_comment(comments[index].as_string().unwrap());
            tb_builder.add_header(hb);
        }
    }
    // client side code
}

