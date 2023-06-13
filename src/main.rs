// use office::Excel;

use std::env;
use std::error::Error;
use std::fs::File;
use std::ops::Not;
use std::path::Path;

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use crate::context::func::ConfigExporter;
use crate::context::model::{ConfigHeaderBuilder, ConfigTable, ConfigTableBuilder};
use crate::export::exporter::CsvExporter;

mod context;
mod export;

fn main() {
    // let args = env::args();
    println!("Hello, world!");
    // let mut excel = Excel::open("config/file.xlsx").unwrap();
    // let r = excel.worksheet_range("Sheet1").unwrap();
    // for row in r.rows() {
    //     println!("row={:?}, row[0]={:?}", row, row[0]);
    // }
    let file_path = Path::new("config/RhythmMasterConfig.xlsx");
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
    let c = CsvExporter {};
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


    let header = ConfigHeaderBuilder::new()
        .set_index(1234)
        .set_comment(String::from("comment"))
        .set_field_name(String::from("nnn"))
        .set_field_type(String::from("ttt"))
        .build();
    println!("{:?}", header);
}

// fn example() -> Result<(), Error> {
//     let path = format!("{}/tests/temperature.xlsx", env!("CARGO_MANIFEST_DIR"));
//     let mut workbook: Xlsx<_> = open_workbook(path)?;
//     let range = workbook.worksheet_range("Sheet1")
//         .ok_or(Error::Msg("Cannot find 'Sheet1'"))??;
//
//     let mut iter = RangeDeserializerBuilder::new().from_range(&range)?;
//
//     if let Some(result) = iter.next() {
//         let (label, value): (String, f64) = result?;
//         assert_eq!(label, "celsius");
//         assert_eq!(value, 22.2222);
//         Ok(())
//     } else {
//         Err(From::from("expected at least one record but got none"))
//     }
// }