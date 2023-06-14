use std::error::Error;
use crate::args::BuildArgs;

use crate::context::model::ConfigTable;

pub trait ConfigLoader {
    fn load(&self, args: &BuildArgs, path: &str, callback: fn(&ConfigTable));
}

pub trait ConfigExporter {
    fn export(&self, dir: &str, t: &ConfigTable) -> bool;
    fn export_with_default_dir(&self, t: &ConfigTable) -> bool {
        self.export(".", t)
    }
}

pub trait LangExporter {
    fn gen(&self, dir: &str, t: &ConfigTable) -> Result<bool, &str>;
}