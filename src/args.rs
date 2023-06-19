use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use clap::builder::{PossibleValue, Str, TypedValueParser};
use clap::{Args, Parser, ValueEnum};
use crate::args::BuildMode::{Client, Server};
use crate::args::LoadMode::{AllSheets, FirstSheet};

use crate::lang::Lang;

#[derive(Default, Debug)]
pub struct BuildArgs {
    /// 构建模式 server/client；决定配置表头加载，配置数据筛选
    pub mode: BuildMode,
    /// 配置文件路径，可以是文件或目录
    pub path: String,
    /// 配置加载模式，单sheet或全部sheet
    pub load: LoadMode,
    /// 配置头解析——注释行
    pub comment_pattern: LinePattern,
    /// 配置头解析——成员类型
    pub type_pattern: LinePattern,
    /// 配置头解析——成员名
    pub name_pattern: LinePattern,
    /// 配置头解析——mode行
    pub mode_pattern: LinePattern,
    /// 配置输出类型
    pub file_type: ConfigExportFileType,
    /// 配置输出参数
    pub config_export: ExportArgs,
    /// 输出代码package，为有package的语言提供
    pub pkg: String,
    /// 输出代码目标语言
    pub lang: Lang,
    /// 代码输出参数
    pub lang_export: ExportArgs,
}

impl BuildArgs {
    pub fn default_naming_func(filename: &str, sheet_name: &str) -> String {
        format!("{}_{}", filename, sheet_name)
    }
    pub fn default_lp_extractor(line: &str) -> String {
        return line.to_string();
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
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

// impl FromStr for BuildMode {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "server" => Ok(Server),
//             "client" => Ok(Client),
//             _ => Err(format!("unknown build mode {s}")),
//         }
//     }
// }

impl ValueEnum for BuildMode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Server, Client]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Server => PossibleValue::new("server").help("配置表构建模式=server，决定配置表头读取和数据筛选"),
            Client => PossibleValue::new("client").help("配置表构建模式=client，决定配置表头读取和数据筛选"),
        })
    }
}

#[derive(Default, PartialEq, Eq, Debug, Copy, Clone)]
pub enum LoadMode {
    #[default]
    AllSheets,
    FirstSheet,
}

impl ValueEnum for LoadMode {
    fn value_variants<'a>() -> &'a [Self] {
        &[AllSheets, FirstSheet]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            AllSheets => PossibleValue::new("all").help("配置表加载模式，读取全部工作簿"),
            FirstSheet => PossibleValue::new("first").help("配置表加载模式，读取第一个工作簿"),
        })
    }
}

// #[derive(Clone, Args)]
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
            extractor: BuildArgs::default_lp_extractor,
        }
    }
}

impl Debug for LinePattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name:{} line_no:{}", self.name, self.line_no)
    }
}

#[derive(Clone)]
pub struct ExportArgs {
    pub out_dir: String,
    pub naming_func: fn(filename: &str, sheet_name: &str) -> String,
}

impl Default for ExportArgs {
    fn default() -> Self {
        Self {
            out_dir: String::from("."),
            naming_func: BuildArgs::default_naming_func,
        }
    }
}

impl Debug for ExportArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "out_dir:{}", self.out_dir)
    }
}

impl ExportArgs {
    pub fn gen_config_name(&self, filename: &str, sheet_name: &str, file_type: &ConfigExportFileType) -> String {
        let name = (self.naming_func)(filename, sheet_name);
        let suffix = file_type.to_string();
        name + "." + suffix.as_str()
    }

    pub fn gen_lang_name(&self, filename: &str, sheet_name: &str, lang_type: &Lang) -> String {
        let name = (self.naming_func)(filename, sheet_name);
        let suffix = lang_type.to_string();
        if suffix.len() > 0 {
            return name + "." + suffix.as_str();
        }
        name
    }
}

#[derive(Default, Debug, Clone)]
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

// #[derive(Debug, Parser)]
// #[command(author, version, about)]
pub struct BuildArgsBuilder {
    // #[arg(short, long)]
    mode: Option<BuildMode>,
    // #[arg(short, long)]
    path: Option<String>,
    // #[arg(short, long)]
    load: Option<LoadMode>,
    comment_pattern: Option<LinePattern>,
    type_pattern: Option<LinePattern>,
    name_pattern: Option<LinePattern>,
    mode_pattern: Option<LinePattern>,
    file_type: Option<ConfigExportFileType>,
    config_export: Option<ExportArgs>,
    pkg: Option<String>,

    lang: Option<Lang>,
    lang_export: Option<ExportArgs>,
}

impl BuildArgsBuilder {
    pub fn new() -> Self {
        Self {
            mode: None,
            path: None,
            load: None,
            comment_pattern: None,
            type_pattern: None,
            name_pattern: None,
            mode_pattern: None,
            file_type: None,
            config_export: None,
            pkg: None,
            lang: None,
            lang_export: None,
        }
    }

    pub fn set_build_mode(mut self, mode: BuildMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn set_config_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
    pub fn set_config_load_mode(mut self, load: LoadMode) -> Self {
        self.load = Some(load);
        self
    }
    pub fn set_config_comment_pattern(mut self, comment_pattern: LinePattern) -> Self {
        self.comment_pattern = Some(comment_pattern);
        self
    }
    pub fn set_config_type_pattern(mut self, type_pattern: LinePattern) -> Self {
        self.type_pattern = Some(type_pattern);
        self
    }
    pub fn set_config_name_pattern(mut self, name_pattern: LinePattern) -> Self {
        self.name_pattern = Some(name_pattern);
        self
    }
    pub fn set_config_mode_pattern(mut self, mode_pattern: LinePattern) -> Self {
        self.mode_pattern = Some(mode_pattern);
        self
    }
    pub fn set_config_file_type(mut self, file_type: ConfigExportFileType) -> Self {
        self.file_type = Some(file_type);
        self
    }
    pub fn set_config_export(mut self, config_export: ExportArgs) -> Self {
        self.config_export = Some(config_export);
        self
    }
    pub fn set_pkg(mut self, pkg: String) -> Self {
        self.pkg = Some(pkg);
        self
    }
    pub fn set_lang(mut self, lang: Lang) -> Self {
        self.lang = Some(lang);
        self
    }
    pub fn set_lang_export(mut self, lang_export: ExportArgs) -> Self {
        self.lang_export = Some(lang_export);
        self
    }

    pub fn build(mut self) -> BuildArgs {
        BuildArgs {
            mode: self.mode.unwrap_or_default(),
            path: self.path.unwrap_or(".".to_string()),
            load: self.load.unwrap_or_default(),
            comment_pattern: self.comment_pattern.unwrap_or_default(),
            type_pattern: self.type_pattern.unwrap_or_default(),
            name_pattern: self.name_pattern.unwrap_or_default(),
            mode_pattern: self.mode_pattern.unwrap_or_default(),
            file_type: self.file_type.unwrap_or_default(),
            config_export: self.config_export.unwrap_or_default(),
            pkg: self.pkg.unwrap_or("cfg".to_string()),
            lang: self.lang.unwrap_or_default(),
            lang_export: self.lang_export.unwrap_or_default(),
        }
    }
}

pub struct LinePatternBuilder {
    name: Option<String>,
    line_no: Option<i32>,
    extractor: Option<fn(input: &str) -> String>,
}

impl LinePatternBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            line_no: None,
            extractor: None,
        }
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn set_line_no(mut self, line_no: i32) -> Self {
        self.line_no = Some(line_no);
        self
    }
    pub fn set_extractor(mut self, extractor: fn(input: &str) -> String) -> Self {
        self.extractor = Some(extractor);
        self
    }
    pub fn build(mut self) -> LinePattern {
        LinePattern {
            name: self.name.unwrap_or_default(),
            line_no: self.line_no.unwrap_or_default(),
            extractor: self.extractor.unwrap_or(|s| { s.to_string() }),
        }
    }
}

pub struct ExtractArgsBuilder {
    out_dir: Option<String>,
    naming_func: Option<fn(filename: &str, sheet_name: &str) -> String>,
}

impl ExtractArgsBuilder {
    pub fn new() -> Self {
        Self {
            out_dir: None,
            naming_func: None,
        }
    }
    pub fn set_out_dir(mut self, out_dir: String) -> Self {
        self.out_dir = Some(out_dir);
        self
    }
    pub fn set_naming_func(mut self, func: fn(filename: &str, sheet_name: &str) -> String) -> Self {
        self.naming_func = Some(func);
        self
    }

    pub fn build(mut self) -> ExportArgs {
        ExportArgs {
            out_dir: self.out_dir.unwrap_or_default(),
            naming_func: self.naming_func.unwrap_or(BuildArgs::default_naming_func),
        }
    }
}

// fn parse_key_val<T>(s: &str) -> Result<(T), Box<dyn Error + Send + Sync + 'static>>
//     where
//         T: std::str::FromStr,
//         T::Err: Error + Send + Sync + 'static,
// {
//     let pos = s
//         .find('=')
//         .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
//     Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
// }