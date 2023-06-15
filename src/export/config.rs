use calamine::DataType;
use calamine::DataType::String;
use crate::args::{BuildArgs, LoadMode};

use crate::context::func::ConfigExporter;
use crate::context::model::ConfigTable;

pub struct CsvExporter {}

impl CsvExporter {
    pub fn new() -> CsvExporter {
        CsvExporter {}
    }
}

impl ConfigExporter for CsvExporter {
    fn export(&self, args: &BuildArgs, t: &ConfigTable) -> bool {
        let mut sheet_name = "";
        if args.load == LoadMode::AllSheets {
            sheet_name = t.sheet_name.as_str();
        }
        let filename = (args.lang_export.naming.func)(t.name.replace(".xlsx", "").as_str(), sheet_name);
        let path = format!("{}/{}.csv", args.lang_export.out_dir, filename);
        let mut writer = csv::Writer::from_path(path).unwrap();
        for d in &t.data {
            writer.write_record(d.iter().map(|v| v.as_string().unwrap_or_default()).collect::<Vec<_>>().as_slice()).unwrap();
        }
        match writer.flush() {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Error write csv file:{}.{}", t.name, err);
                false
            }
        }
    }
}

pub struct JsonExporter {}

pub struct SqlExporter {}