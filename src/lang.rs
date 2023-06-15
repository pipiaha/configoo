#[derive(Default)]
pub enum Lang{
    Java,
    CSharp,
    #[default]
    Go,
    Lua,
    Custom,
}