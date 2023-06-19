use std::str::FromStr;

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

impl FromStr for Types {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(Types::Int),
            "ints" => Ok(Types::Ints),
            "float" => Ok(Types::Float),
            "floats" => Ok(Types::Floats),
            "double" => Ok(Types::Double),
            "doubles" => Ok(Types::Doubles),
            "byte" => Ok(Types::Byte),
            "bytes" => Ok(Types::Bytes),
            "bool" => Ok(Types::Bool),
            "bools" => Ok(Types::Bools),
            "string" => Ok(Types::String),
            "strings" => Ok(Types::Strings),
            "json" => Ok(Types::Json),
            &_ => { Err(format!("config type not supported.{}", s)) }
        }
    }
}