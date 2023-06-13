use std::error::Error;

use crate::context::model::ConfigTable;

pub trait ConfigLoader {
    fn load(path: String) -> ConfigTable;
}

pub trait ConfigExporter {
    fn export(&self, dir: &str, t: &ConfigTable) -> bool;
    fn export_with_default_dir(&self, t: &ConfigTable) -> bool {
        self.export(".", t)
    }
}

pub trait CodeExporter {
    fn gen(&self, dir: &str, t: &ConfigTable) -> Result<bool, dyn Error>;
}