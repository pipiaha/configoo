#[derive(Default, Debug, Clone)]
pub enum Lang {
    Java,
    CSharp,
    #[default]
    Go,
    Lua,
    Typescript,
    Javascript,
    Dart,
    Rust,
    Custom,
}

impl ToString for Lang {
    fn to_string(&self) -> String {
        let typ = match self {
            Lang::Java => { "java" }
            Lang::CSharp => { "cs" }
            Lang::Go => { "go" }
            Lang::Lua => { "lua" }
            Lang::Typescript => { "ts" }
            Lang::Javascript => { "js" }
            Lang::Dart => { "dart" }
            Lang::Rust => { "rs" }
            Lang::Custom => { "" }
        };
        typ.to_string()
    }
}

/// configoo配置表 支持的数据类型
pub enum Types {
    Int,
    Ints,
    Float,
    Floats,
    Double,
    Doubles,
    Byte,
    Bytes,
    Bool,
    Bools,
    String,
    Strings,
    Json,
}