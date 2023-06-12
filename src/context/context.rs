pub trait ConfigLoader {
    fn load(path: String) -> ConfigTable;
}

pub trait ConfigExporter<'a, T> {
    fn export(t: &ConfigTable) -> T;
}