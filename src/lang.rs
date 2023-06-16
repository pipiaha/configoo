#[derive(Default, Debug)]
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