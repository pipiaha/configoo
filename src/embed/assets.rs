use rust_embed::RustEmbed;

use crate::lang::Lang;

// https://zhuanlan.zhihu.com/p/563806378

#[derive(RustEmbed)]
#[folder = "src/embed/template"]
struct TemplateAsset;

pub fn get_template(lang: &Lang) -> Option<rust_embed::EmbeddedFile> {
    if lang.to_string().len() <= 0 {
        return None;
    }
    TemplateAsset::get(format!("{}.template", lang.to_string()).as_str())
}
