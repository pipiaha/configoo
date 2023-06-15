use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

// TODO error
// 模板参数
#[derive(Serialize, Deserialize)]
pub struct LangTemplateData {
    pub pkg: String,
    pub name: String,
    pub fields: Vec<LangFieldData>,
}

#[derive(Serialize, Deserialize)]
pub struct LangFieldData {
    pub field_name: String,
    pub field_type: String,
    pub field_comment: String,
}
// 类型支持和转换