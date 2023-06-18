// use crate::context::func::LangExporter;
// use crate::context::model::{Context, LangTemplateData};
// use crate::export::lang::BaseLangWriter;
//
// pub struct GolangExporter {
//     writer: BaseLangWriter,
// }
//
// impl GolangExporter {
//     pub fn new() -> Box<dyn LangExporter> {
//         Box::new(Self { writer: BaseLangWriter })
//     }
// }
//
// impl LangExporter for GolangExporter {
//     fn gen(&self, ctx: &Context, data: &LangTemplateData, src: &str) -> Option<&str> {
//         self.writer.gen(ctx, data, src)
//     }
// }

