// config header cell unit
pub struct ConfigHeader {
    index: i32,
    server_field_name: String,
    server_field_type: String,
    comment: String,
}

// config column unit
pub struct ConfigColumn {
    header: *ConfigHeader,
    value: String,
    row_index: i32,
}

// config row
pub struct ConfigRow {
    row_index: i32,
    data: Vec<*ConfigColumn>,
}

// whole config data
pub struct ConfigTable {
    name: String,
    data: Vec<*ConfigRow>,
    header: Vec<*ConfigHeader>,
}