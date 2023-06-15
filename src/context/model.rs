use std::fmt::{Display, Formatter};

use calamine::DataType;

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

// config column unit
#[derive(Default, Debug)]
pub struct ConfigColumn {
    header: ConfigHeader,
    value: String,
    row_index: i32,
}

pub struct ConfigColumnBuilder {
    header: Option<ConfigHeader>,
    value: Option<String>,
    row_index: Option<i32>,
}

impl ConfigColumnBuilder {
    pub fn new() -> Self {
        Self {
            header: None,
            value: None,
            row_index: None,
        }
    }

    pub fn set_header(mut self, header: ConfigHeader) -> Self {
        self.header = Some(header);
        self
    }
    pub fn set_value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
    pub fn set_row_index(mut self, row_index: i32) -> Self {
        self.row_index = Some(row_index);
        self
    }

    pub fn build(self) -> ConfigColumn {
        ConfigColumn {
            header: self.header.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            row_index: self.row_index.unwrap_or_default(),
        }
    }
}

// config row
#[derive(Default, Debug)]
pub struct ConfigRow {
    row_index: i32,
    data: Vec<ConfigColumn>,
}

pub struct ConfigRowBuilder {
    row_index: Option<i32>,
    data: Option<Vec<ConfigColumn>>,
}

impl ConfigRowBuilder {
    pub fn new() -> ConfigRowBuilder {
        return ConfigRowBuilder {
            row_index: None,
            data: None,
        };
    }

    pub fn set_row_index(mut self, idx: i32) -> Self {
        self.row_index = Some(idx);
        self
    }

    pub fn add_column(mut self, col: ConfigColumn) -> Self {
        self.data.get_or_insert_with(Vec::new).push(col);
        self
    }

    pub fn build(self) -> ConfigRow {
        ConfigRow {
            row_index: self.row_index.unwrap_or_default(),
            data: self.data.unwrap_or_default(),
        }
    }
}

// whole config data
#[derive(Default, Debug)]
pub struct ConfigTable {
    pub name: String,
    pub sheet_name: String,
    pub data: Vec<Vec<DataType>>,
    pub header: Vec<ConfigHeader>,
}

pub struct ConfigTableBuilder {
    name: Option<String>,
    sheet_name: Option<String>,
    data: Option<Vec<Vec<DataType>>>,
    header: Option<Vec<ConfigHeader>>,
}

impl ConfigTableBuilder {
    pub fn new() -> ConfigTableBuilder {
        ConfigTableBuilder {
            name: None,
            sheet_name: None,
            data: None,
            header: None,
        }
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
            name: self.name.unwrap_or_default(),
            sheet_name: self.sheet_name.unwrap_or_default(),
            data: self.data.unwrap_or_default(),
            header: self.header.unwrap_or_default(),
        }
    }
}