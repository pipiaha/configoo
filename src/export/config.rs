use std::fs;
use std::ops::Not;
use std::path::Path;
use calamine::DataType;
use calamine::DataType::String;
use crate::args::{BuildArgs, ConfigExportFileType, ExportFileNaming, LoadMode};

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
        let dir = Path::new(args.lang_export.out_dir.as_str());
        if !dir.exists() {
            match fs::create_dir(dir) {
                Ok(_) => {}
                Err(err) => { eprintln!("Error export config {},{}", t.name, t.sheet_name) }
            };
        }
        let filename = args.lang_export.naming.gen_config_name(t.name.replace(".xlsx", "").as_str(), sheet_name, ConfigExportFileType::Csv);
        let path = format!("{}/{}", args.lang_export.out_dir, filename);
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