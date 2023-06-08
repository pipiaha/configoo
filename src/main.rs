// use office::Excel;

use std::path::Path;
use calamine::{open_workbook, Xlsx};

fn main() {
    println!("Hello, world!");
    // let mut excel = Excel::open("config/file.xlsx").unwrap();
    // let r = excel.worksheet_range("Sheet1").unwrap();
    // for row in r.rows() {
    //     println!("row={:?}, row[0]={:?}", row, row[0]);
    // }

    let file_path = Path::new("config/file.xlsx");
    // 打开 XLSX 文件
    let mut workbook: Xlsx<_> = match open_workbook(file_path) {
        Ok(wb) => workbook = wb,
        Err(e) => {
            eprintln!("Failed to open workbook: {}", e);
        }
    };
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
    for row in sheet.rows() {
        println!("row={:?}, row[0]={:?}", row, row[0]);
    }
}