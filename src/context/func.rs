use crate::args::BuildArgs;

use crate::context::model::ConfigTable;

pub trait ConfigLoader {
    fn load<Func>(&self, args: &BuildArgs, path: &str, callback: Func)
        where Func: Fn(&ConfigTable);
}

pub trait ConfigExporter {
    fn export(&self, args: &BuildArgs, t: &ConfigTable) -> bool;// TODO return Option
}

pub trait LangExporter {
    fn gen(&self, args: &BuildArgs, t: &ConfigTable) -> Option<&str>;
}