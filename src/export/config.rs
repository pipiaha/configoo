use std::fs;
use std::path::Path;

use crate::args::{BuildArgs, ConfigExportFileType, LoadMode};
use crate::context::func::ConfigExporter;
use crate::context::model::{ConfigTable, Context};

pub struct CsvExporter {}

// TODO 需要Export总入口

impl CsvExporter {
    pub fn new() -> CsvExporter {
        CsvExporter {}
    }
}

impl ConfigExporter for CsvExporter {
    fn export(&self, ctx: &Context) -> bool {
        let mut sheet_name = "";
        let t = ctx.tb;
        let args = ctx.args;
        if args.load == LoadMode::AllSheets {
            sheet_name = t.sheet_name.as_str();
        }
        let dir = Path::new(args.config_export.out_dir.as_str());
        if !dir.exists() {
            match fs::create_dir_all(dir) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error export config {},{}.{}", t.name, t.sheet_name, err)
                }
            };
        }
        let filename = args.config_export.gen_config_name(
            t.name.replace(".xlsx", "").as_str(),
            sheet_name,
            &ConfigExportFileType::Csv,
        );
        let path = format!("{}/{}", args.config_export.out_dir, filename);
        let mut writer = csv::Writer::from_path(path).unwrap();
        for d in &t.data {
            writer.write_record(
                d.iter()
                    .map(|v| v.as_string().unwrap_or_default())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ).unwrap();
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
