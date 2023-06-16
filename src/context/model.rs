use std::fmt::{Display, Formatter};
use calamine::DataType;
use serde::{Deserialize, Serialize};

use crate::args::BuildArgs;

#[derive(Debug)]
pub struct Context<'args, 'tb> {
    pub args: &'args BuildArgs,
    pub tb: &'tb ConfigTable,
}

impl<'args, 'tb> Context<'args, 'tb> {
    pub fn new(args: &'args BuildArgs, tb: &'tb ConfigTable) -> Context<'args, 'tb> {
        Self { args, tb }
    }
}

// config header cell unit
#[derive(Default, Debug)]
pub struct ConfigHeader {
    index: i32,
    field_name: String,
    field_type: String,
    comment: String,
}

pub struct ConfigHeaderBuilder {
    index: Option<i32>,
    field_name: Option<String>,
    field_type: Option<String>,
    comment: Option<String>,
}

impl ConfigHeaderBuilder {
    pub fn new() -> Self {
        Self {
            index: None,
            field_name: None,
            field_type: None,
            comment: None,
        }
    }
    pub fn set_index(mut self, index: i32) -> Self {
        self.index = Some(index);
        self
    }
    pub fn set_field_name(mut self, field_name: String) -> Self {
        self.field_name = Some(field_name);
        self
    }
    pub fn set_field_type(mut self, field_type: String) -> Self {
        self.field_type = Some(field_type);
        self
    }
    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
    pub fn build(self) -> ConfigHeader {
        ConfigHeader {
            index: self.index.unwrap_or_default(),
            field_name: self.field_name.unwrap_or_default(),
            field_type: self.field_type.unwrap_or_default(),
            comment: self.comment.unwrap_or_default(),
        }
    }
}

// whole config data
#[derive(Default, Debug)]
pub struct ConfigTable {
    pub pkg: String,
    pub name: String,
    pub sheet_name: String,
    pub data: Vec<Vec<DataType>>,
    pub header: Vec<ConfigHeader>,
}

pub struct ConfigTableBuilder {
    pkg: Option<String>,
    name: Option<String>,
    sheet_name: Option<String>,
    data: Option<Vec<Vec<DataType>>>,
    header: Option<Vec<ConfigHeader>>,
}

impl ConfigTableBuilder {
    pub fn new() -> ConfigTableBuilder {
        ConfigTableBuilder {
            pkg: None,
            name: None,
            sheet_name: None,
            data: None,
            header: None,
        }
    }

    pub fn set_pkg(mut self, pkg: String) -> Self {
        self.pkg = Some(pkg);
        self
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn set_sheet_name(mut self, sheet_name: String) -> Self {
        self.sheet_name = Some(sheet_name);
        self
    }

    pub fn add_header(mut self, hb: ConfigHeaderBuilder) -> Self {
        self.header.get_or_insert_with(|| Vec::new()).push(hb.build());
        self
    }

    pub fn add_data(mut self, row: Vec<DataType>) -> Self {
        self.data.get_or_insert_with(|| Vec::new()).push(row);
        self
    }

    pub fn build(self) -> ConfigTable {
        ConfigTable {
            pkg: self.pkg.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            sheet_name: self.sheet_name.unwrap_or_default(),
            data: self.data.unwrap_or_default(),
            header: self.header.unwrap_or_default(),
        }
    }
}

// 导出文件参数
pub struct ConfigData {
    pub filename: String,
    pub out_dir: String,
    pub content: String,
}

// 模板参数
#[derive(Serialize, Deserialize, Clone)]
pub struct LangTemplateData {
    pub pkg: String,
    pub name: String,
    pub fields: Vec<LangFieldData>,
}

impl LangTemplateData {
    fn from_table(tb: &ConfigTable) -> LangTemplateData {
        Self {
            pkg: tb.pkg.clone(),
            name: "".to_string(),
            fields: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LangFieldData {
    pub field_name: String,
    pub field_type: String,
    pub field_comment: String,
}