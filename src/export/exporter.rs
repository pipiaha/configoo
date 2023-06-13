use calamine::DataType;
use calamine::DataType::String;

use crate::context::func::ConfigExporter;
use crate::context::model::ConfigTable;

pub struct CsvExporter {}

impl ConfigExporter for CsvExporter {
    fn export(&self, dir: &str, t: &ConfigTable) -> bool {
        let mut writer = csv::Writer::from_path(&t.name).unwrap();
        for d in &t.data {
            writer.write_record(d.iter().map(|v| v.as_string().unwrap()).collect::<Vec<_>>().as_slice()).unwrap();
        }
        match writer.flush() {
            Ok(_) => true,
            Err(err) => { false }
        }
    }
}

pub struct JsonExporter{}

pub struct SqlExporter{}