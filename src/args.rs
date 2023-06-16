use std::fmt::{Debug, Formatter};
use crate::lang::Lang;

#[derive(Default, Debug)]
pub struct BuildArgs {
    pub mode: BuildMode,
    pub path: String,
    pub lang: Lang,
    pub load: LoadMode,
    pub comment_pattern: LinePattern,
    pub type_pattern: LinePattern,
    pub name_pattern: LinePattern,
    pub mode_pattern: LinePattern,
    pub config_export: ExportArgs,
    pub lang_export: ExportArgs,
}

#[derive(Default, Debug)]
pub enum BuildMode {
    #[default]
    Server,
    Client,
}

impl ToString for BuildMode {
    fn to_string(&self) -> String {
        match self {
            BuildMode::Server => { "server".to_string() }
            BuildMode::Client => { "client".to_string() }
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub enum LoadMode {
    #[default]
    AllSheets,
    FirstSheet,
}

pub struct LinePattern {
    pub name: String,
    pub line_no: i32,
    pub extractor: fn(input: &str) -> String,
}

impl Default for LinePattern {
    fn default() -> Self {
        Self {
            name: Default::default(),
            line_no: Default::default(),
            extractor: |t| String::from(t),
        }
    }
}

impl Debug for LinePattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name:{} line_no:{}", self.name, self.line_no)
    }
}

pub struct ExportArgs {
    pub out_dir: String,
    pub naming: ExportFileNaming,
}

impl Default for ExportArgs {
    fn default() -> Self {
        Self {
            out_dir: String::from("."),
            naming: Default::default(),
        }
    }
}

impl Debug for ExportArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "out_dir:{}", self.out_dir)
    }
}

#[derive(Debug)]
pub struct ExportFileNaming {
    pub func: fn(filename: &str, sheet_name: &str) -> String,
    pub file_type: ConfigExportFileType,
}

impl Default for ExportFileNaming {
    fn default() -> Self {
        Self {
            func: |f, s| format!("{}_{}", f, s),
            file_type: Default::default(),
        }
    }
}

impl ExportFileNaming {
    pub fn gen_config_name(&self, filename: &str, sheet_name: &str, file_type: ConfigExportFileType) -> String {
        let name = (self.func)(filename, sheet_name);
        let suffix = file_type.to_string();
        name + "." + suffix.as_str()
    }

    pub fn gen_lang_name(&self, filename: &str, sheet_name: &str, lang_type: Lang) -> String {
        let name = (self.func)(filename, sheet_name);
        let suffix = lang_type.to_string();
        if suffix.len() > 0 {
            return name + "." + suffix.as_str();
        }
        name
    }
}

#[derive(Default, Debug)]
pub enum ConfigExportFileType {
    #[default]
    Csv,
    Sql,
    Json,
}

impl ToString for ConfigExportFileType {
    fn to_string(&self) -> String {
        let typ = match self {
            ConfigExportFileType::Csv => { "csv" }
            ConfigExportFileType::Sql => { "sql" }
            ConfigExportFileType::Json => { "json" }
        };
        typ.to_string()
    }
}