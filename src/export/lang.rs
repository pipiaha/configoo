// use std::fs;
// use std::path::Path;
//
// use tera::Tera;
//
// use crate::context::func::{LangExporter, LangExporter, LangTemplateDataModifier, LangWriter};
// use crate::context::model::{Context, LangFieldData, LangTemplateData};
// use crate::embed;
// use crate::lang::Lang;
//
// pub struct BaseLangLifetime {
//     modifiers: Vec<LangTemplateDataModifier>,
//     writer: LangWriter,
//     // inner: Box<dyn LangExporter>,
// }
//
// impl BaseLangLifetime {
//     // pub fn wrap(lang_exp: Box<dyn LangExporter>) -> Self {
//     //     Self {
//     //         modifiers: vec![],
//     //         // inner: lang_exp,
//     //     }
//     // }
//     pub fn new() -> Self {
//         Self {
//             modifiers: vec![],
//             writer:BaseLangWriter,
//         }
//     }
//     fn modify(&self, ctx: &Context, data: &mut LangTemplateData) {
//         self.modifiers.iter().for_each(|m| (m)(ctx, data))
//     }
// }
//
// // impl LangExporter for BaseLangLifetime {
// //     fn gen(&self, ctx: &Context, data: &LangTemplateData, src: &str) -> Option<&str> {
// //         self.inner.gen(ctx, data, src)
// //     }
// // }
//
// impl LangExporter for BaseLangLifetime {
//     fn add_modifier(&mut self, modifier: LangTemplateDataModifier) {
//         self.modifiers.push(modifier)
//     }
//
//     fn add_writer(&mut self, writer: LangWriter) {
//         todo!()
//     }
//
//     fn gen(&self, ctx: &Context) -> Option<&str> {
//         let mut d = LangTemplateData {
//             filename: "".to_string(),
//             pkg: "".to_string(),
//             name: "".to_string(),
//             fields: vec![],
//             imports: vec![],
//         };
//         self.modify(ctx, &mut d);
//         // test template
//         let lang = &ctx.args.lang;
//         let br = lang.to_string();
//         let lang_str = br.as_str();
//
//         let f = embed::assets::get_template(&lang).unwrap();
//         let s = std::str::from_utf8(f.data.as_ref()).unwrap();
//         let mut tera = Tera::default();
//         tera.add_raw_template(lang_str, s).unwrap();
//         let mut src_ctx = tera::Context::from_serialize(d.clone()).unwrap();
//         if let Ok(src) = tera.render(lang_str, &src_ctx) {
//             println!("{}", src);
//             return self.inner.gen(ctx, &d, src.as_str());
//         };
//         None
//     }
// }
//
// pub struct BaseLangWriter;
//
// impl LangExporter for BaseLangWriter {
//     fn gen(&self, ctx: &Context, data: &LangTemplateData, src: &str) -> Option<&str> {
//         let out_dir = &ctx.args.lang_export.out_dir;
//         let dir = Path::new(out_dir);
//         if !dir.exists() {
//             fs::create_dir(dir);
//         }
//         match fs::write(format!("{}/{}.{}", out_dir, ctx.tb.name, ctx.args.lang.to_string()), src) {
//             Ok(_) => {}
//             Err(err) => { eprintln!("Error write file {}.{}", data.name, err) }
//         };
//         Some("")
//     }
// }