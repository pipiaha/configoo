use std::fmt::{Display, Formatter};

// config header cell unit
#[derive(Default,Debug)]
pub struct ConfigHeader {
    index: i32,
    server_field_name: String,
    server_field_type: String,
    comment: String,
}

pub struct ConfigHeaderBuilder {
    index: Option<i32>,
    server_field_name: Option<String>,
    server_field_type: Option<String>,
    comment: Option<String>,
}

impl ConfigHeaderBuilder {
    pub fn new() -> Self {
        Self {
            index: None,
            server_field_name: None,
            server_field_type: None,
            comment: None,
        }
    }
    pub fn set_index(mut self, index: i32) -> Self {
        self.index = Some(index);
        self
    }
    pub fn set_server_field_name(mut self, server_field_name: String) -> Self {
        self.server_field_name = Some(server_field_name);
        self
    }
    pub fn set_server_field_type(mut self, server_field_type: String) -> Self {
        self.server_field_type = Some(server_field_type);
        self
    }
    pub fn set_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
    pub fn build(self) -> ConfigHeader {
        ConfigHeader {
            index: self.index.unwrap_or_default(),
            server_field_name: self.server_field_name.unwrap_or_default(),
            server_field_type: self.server_field_type.unwrap_or_default(),
            comment: self.comment.unwrap_or_default(),
        }
    }
}

// config column unit
#[derive(Default,Debug)]
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
#[derive(Default,Debug)]
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
#[derive(Default,Debug)]
pub struct ConfigTable {
    name: String,
    data: Vec<ConfigRow>,
    header: Vec<ConfigHeader>,
}

pub struct ConfigTableBuilder {
    name: Option<String>,
    data: Option<Vec<ConfigRow>>,
    header: Option<Vec<ConfigHeader>>,
}

impl ConfigTableBuilder {
    pub fn new() -> ConfigTableBuilder {
        ConfigTableBuilder {
            name: None,
            data: None,
            header: None,
        }
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}