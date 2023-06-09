// config header cell unit
pub struct ConfigHeader {
    index: i32,
    server_field_name: String,
    server_field_type: String,
    comment: String,
}

// config column unit
pub struct ConfigColumn {
    header: &ConfigHeader,
    value: String,
    row_index: i32,
}

// config row
pub struct ConfigRow {
    row_index: i32,
    data: Vec<ConfigColumn>,
}

pub struct ConfigRowBuilder {
    row_index: Option<i32>,
    data: Option<Vec<ConfigColumn>>,
}

impl ConfigRowBuilder {
    fn new() -> Self {
        Self {
            row_index: None,
            data: None,
        };
    }

    fn set_row_index(mut self, idx: i32) -> Self {
        self.row_index = Some(idx);
        self
    }

    fn add_column(mut self, col: ConfigColumn) -> Self {
        self.data.get_or_insert_with(Vec::new).push(col);
        self
    }

    fn build(self) -> ConfigRow {
        ConfigRow {
            row_index: self.row_index.unwrap_or_default(),
            data: self.data.unwrap_or_default(),
        }
    }
}

// whole config data
pub struct ConfigTable {
    name: String,
    data: Vec<ConfigRow>,
    header: Vec<ConfigHeader>,
}

impl ConfigTable {
    fn new() -> ConfigTable {
        ConfigTable {
            name: "".to_string(),
            data: vec![],
            header: vec![],
        };
    }
}

pub struct ConfigTableBuilder {
    name: Option<String>,
    data: Option<Vec<ConfigRow>>,
    header: Option<Vec<ConfigHeader>>,
}

impl ConfigTableBuilder {
    fn new() -> ConfigTableBuilder {
        ConfigTableBuilder {
            name: None,
            data: None,
            header: None,
        }
    }
    fn set_name(mut self, name: String) -> ConfigTableBuilder {
        self.name = Some(name);
        self;
    }
}
