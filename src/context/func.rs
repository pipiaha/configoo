use std::fs;
use std::ops::Index;
use std::path::Path;

use tera::Tera;

use crate::args::BuildArgs;
use crate::context::model::{ConfigTable, Context, LangTemplateData};
use crate::embed;

pub trait ConfigLoader {
    fn load<Func>(&self, args: &BuildArgs, callback: Func)
        where Func: Fn(&Context);
}

pub trait ConfigExporter {
    fn export(&self, ctx: &Context) -> bool;// TODO return Option
}

// pub trait LangExporter {
//     fn gen(&self, ctx: &Context, data: &LangTemplateData, src: &str) -> Option<&str>;
// }

pub struct LangExporter {
    modifiers: Vec<LangTemplateDataModifier>,
    writer: LangWriter,
    template_loader: LangTemplateLoader,
    // inner: Box<dyn LangExporter>,
}

impl LangExporter {
    fn modify(&self, ctx: &Context, data: &mut LangTemplateData) {
        self.modifiers.iter().for_each(|m| (m)(ctx, data))
    }

    pub fn export(&self, ctx: &Context) -> Option<()> {
        // default template data
        let mut d = LangTemplateData {
            filename: "".to_string(),
            pkg: "".to_string(),
            name: "".to_string(),
            fields: vec![],
            imports: vec![],
        };
        // update template data
        self.modify(ctx, &mut d);

        // render
        let tuple = (self.template_loader)(&ctx);
        let template_name = tuple.0.as_str();
        let s = tuple.1.as_str();

        let mut tera = Tera::default();
        tera.add_raw_template(template_name, s).unwrap();
        let mut src_ctx = tera::Context::from_serialize(d.clone()).unwrap();
        if let Ok(src) = tera.render(template_name, &src_ctx) {
            println!("render completed. template:{}", template_name);
            // write file
            (self.writer)(&ctx, d.filename.as_str(), src.as_str());
            Some("ok")
        } else {
            eprintln!("Error render file. template={},file={}", template_name, &ctx.tb.name);
            None
        };
        None
    }
}

pub type LangTemplateDataModifier = fn(ctx: &Context, data: &mut LangTemplateData);
pub type LangTemplateLoader = fn(ctx: &Context) -> (String, String);
pub type LangWriter = fn(ctx: &Context, filename: &str, src: &str);

pub struct LangExporterBuilder {
    modifiers: Option<Vec<LangTemplateDataModifier>>,
    template_loader: Option<LangTemplateLoader>,
    writer: Option<LangWriter>,
}

impl LangExporterBuilder {
    pub fn new() -> Self {
        Self {
            modifiers: None,
            template_loader: None,
            writer: None,
        }
    }
    pub fn add_modifier(mut self, modifier: LangTemplateDataModifier) -> Self {
        self.modifiers.get_or_insert_with(|| Vec::new()).push(modifier);
        self
    }
    pub fn add_writer(mut self, writer: LangWriter) -> Self {
        self.writer = Some(writer);
        self
    }
    pub fn add_loader(mut self, loader: LangTemplateLoader) -> Self {
        self.template_loader = Some(loader);
        self
    }
    pub fn build(mut self) -> LangExporter {
        LangExporter {
            modifiers: self.modifiers.unwrap_or_default(),
            writer: self.writer.unwrap_or(Self::default_write),
            template_loader: self.template_loader.unwrap_or(Self::default_load),
        }
    }
    pub fn default_write(ctx: &Context, filename: &str, src: &str) {
        let out_dir = &ctx.args.lang_export.out_dir;
        let dir = Path::new(out_dir);
        if !dir.exists() {
            fs::create_dir_all(dir);
        }
        let lang = ctx.args.lang.to_string();
        if filename.ends_with(lang.as_str()) {
            match fs::write(format!("{}/{}", out_dir, filename), src) {
                Ok(_) => {}
                Err(err) => { eprintln!("Error write file {}.{}", filename, err) }
            };
        } else {
            match fs::write(format!("{}/{}.{}", out_dir, filename, lang), src) {
                Ok(_) => {}
                Err(err) => { eprintln!("Error write file {}.{}", filename, err) }
            };
        }
    }
    pub fn default_load(ctx: &Context) -> (String, String) {
        let lang = &ctx.args.lang;
        let lang_str = lang.to_string();

        let f = embed::assets::get_template(&lang).unwrap();
        (lang_str, std::str::from_utf8(f.data.as_ref()).unwrap().to_string())
    }
}