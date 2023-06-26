use std::str::FromStr;
use clap::builder::PossibleValue;

use clap::ValueEnum;

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

impl ValueEnum for Lang {
    fn value_variants<'a>() -> &'a [Self] {
        &[Lang::Java, Lang::CSharp, Lang::Go, Lang::Lua, Lang::Typescript, Lang::Javascript, Lang::Dart, Lang::Rust, Lang::Custom]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Lang::Java =>  PossibleValue::new("java").help("Java"),
            Lang::CSharp =>  PossibleValue::new("cs").help("CSharp"),
            Lang::Go =>  PossibleValue::new("go").help("Go"),
            Lang::Lua =>  PossibleValue::new("lua").help("Lua"),
            Lang::Typescript =>  PossibleValue::new("ts").help("Typescript"),
            Lang::Javascript =>  PossibleValue::new("js").help("Javascript"),
            Lang::Dart =>  PossibleValue::new("dart").help("Dart"),
            Lang::Rust =>  PossibleValue::new("rs").help("Rust"),
            Lang::Custom =>  PossibleValue::new("custom").help("Custom"),
        })
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