use calamine::DataType::String;
use crate::lang::Lang;

#[derive(Default)]
pub struct BuildArgs {
    pub side: BuildSide,
    pub path: String,
    pub lang: Lang,
    pub comment_pattern: LinePattern,
    pub type_pattern: LinePattern,
    pub name_pattern: LinePattern,
    pub side_pattern: LinePattern,
    pub config_export: ExportArgs,
    pub lang_export: ExportArgs,
}

pub enum BuildSide {
    Server,
    Client,
}

#[derive(Default)]
pub struct LinePattern {
    pub name: str,
    pub line_no: i32,
    pub extractor: fn(input: &str) -> &str,
}


#[derive(Default)]
pub struct ExportArgs {
    pub out_dir: str,
    pub naming: ExportFileNaming,
}

#[derive(Default)]
pub struct ExportFileNaming {
    pub naming: fn(filename: &str, sheet_name: &str) -> &str,
    pub file_type: ConfigExportFileType,
}

impl ExportFileNaming {
    pub fn gen_config_name(&self, filename: &str, sheet_name: &str, file_type: ConfigExportFileType) -> String {
        let name = self.naming(filename, sheet_name);
        let suffix = match file_type {
            ConfigExportFileType::Csv => { "csv" }
            ConfigExportFileType::Sql => { "sql" }
            ConfigExportFileType::Json => { "json" }
        };
        name + "." + suffix
    }

    pub fn gen_lang_name(&self, filename: &str, sheet_name: &str, lang_type: Lang) -> String {
        let name = self.naming(filename, sheet_name);
        let suffix = match lang_type {
            Lang::Java => { "java" }
            Lang::CSharp => { "cs" }
            Lang::Go => { "go" }
            Lang::Lua => { "lua" }
            Lang::Custom => { "custom" }// TODO error
        };
        name + "." + suffix
    }
}

pub enum ConfigExportFileType {
    Csv,
    Sql,
    Json,
}