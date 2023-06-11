trait ConfigLoader {
    fn load(path: String) -> ConfigTable;
}