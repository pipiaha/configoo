use crate::args::BuildArgs;
use crate::context::model::{ConfigTable, Context, LangTemplateData};

pub trait ConfigLoader {
    fn load<Func>(&self, args: &BuildArgs, path: &str, callback: Func)
        where Func: Fn(&Context);
}

pub trait ConfigExporter {
    fn export(&self, ctx: &Context) -> bool;// TODO return Option
}

pub trait LangExporter {
    fn gen(& self, ctx: &Context, data: &LangTemplateData, src: & str) -> Option<&str>;
}

pub trait LangBuildLifetime {
    fn add_modifier(&mut self, modifier: LangTemplateDataModifier);
    fn modify(&self, ctx: &Context, data: &mut LangTemplateData);
    fn gen(&self, ctx: &Context) -> Option<&str>;
}

pub type LangTemplateDataModifier = fn(ctx: &Context, data: &mut LangTemplateData);
// modifier provider