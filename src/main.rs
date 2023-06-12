// use office::Excel;

use std::error::Error;
use std::path::Path;

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};

mod context;

fn main() {
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

    let comment_row_index = 1;
    let server_type_row_index = 2;
    let client_type_row_index = 3;
    let server_name_row_index = 4;
    let client_name_row_index = 4;

    let mut index = 0;
    for row in sheet.rows() {
        println!("row={:?}, row[0]={:?}", row, row[0]);
        if index==comment_row_index {}
        index += 1;
    }
    let header = context::model::ConfigHeaderBuilder::new()
        .set_index(1234)
        .set_comment(String::from("comment"))
        .set_server_field_name(String::from("nnn"))
        .set_server_field_type(String::from("ttt"))
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