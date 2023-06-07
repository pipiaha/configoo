use std::any::type_name;

use office::Excel;

fn main() {
    println!("Hello, world!");
    let mut excel = Excel::open("file.xlsx").unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();
    for row in r.rows() {
        println!("row={:?}, row[0]={:?}", row, row[0]);
    }
}