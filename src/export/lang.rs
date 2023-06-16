use tera::Tera;

use crate::context::func::{LangBuildLifetime, LangExporter, LangTemplateDataModifier};
use crate::context::model::{Context, LangFieldData, LangTemplateData};
use crate::embed;
use crate::lang::Lang;

pub struct BaseLangExporter {
    modifiers: Vec<LangTemplateDataModifier>,
    inner: Box<dyn LangExporter>,
}

impl BaseLangExporter {
    pub fn wrap(lang_exp: Box<dyn LangExporter>) -> Self {
        Self {
            modifiers: vec![],
            inner: lang_exp,
        }
    }
}

impl LangExporter for BaseLangExporter {
    fn gen(&self, ctx: &Context, data: &LangTemplateData, src: &str) -> Option<&str> {
        self.inner.gen(ctx, data, src)
    }
}

impl LangBuildLifetime for BaseLangExporter {
    fn add_modifier(&mut self, modifier: LangTemplateDataModifier) {
        self.modifiers.push(modifier)
    }

    fn modify(&self, ctx: &Context, data: &mut LangTemplateData) {
        self.modifiers.iter().for_each(|m| (m)(ctx, data))
    }

    fn gen(&self, ctx: &Context) -> Option<&str> {
        let mut d = LangTemplateData {
            pkg: "".to_string(),
            name: "".to_string(),
            fields: vec![],
        };
        self.modify(ctx, &mut d);
        // test template
        let lang = &ctx.args.lang;
        let br = lang.to_string();
        let lang_str = br.as_str();

        let f = embed::assets::get_template(&lang).unwrap();
        let s = std::str::from_utf8(f.data.as_ref()).unwrap();
        let mut tera = Tera::default();
        tera.add_raw_template(lang_str, s).unwrap();
        let mut src_ctx = tera::Context::from_serialize(d.clone()).unwrap();
        src_ctx.insert("obj", "abcd");
        if let Ok(src) = tera.render(lang_str, &src_ctx) {
            println!("{}", src);
            return self.inner.gen(ctx, &d, src.as_str());
        };
        None
    }
}


pub struct GolangExporter;

impl LangExporter for GolangExporter {
    fn gen(&self, ctx: &Context, data: &LangTemplateData, src: &str) -> Option<&str> {
        let data = LangTemplateData {
            pkg: "abcdefg".to_string(),
            name: "ABAB".to_string(),
            fields: vec![LangFieldData {
                field_name: "f1".to_string(),
                field_type: "string".to_string(),
                field_comment: "ayyyy".to_string(),
            }],
        };

        Some("")
    }
}