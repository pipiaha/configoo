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

use std::str::FromStr;

use crate::context::func::{LangExporterBuilder, LangExporterBuilderCustomizer};
use crate::context::model::{Context, LangFieldData};
use crate::lang;
use crate::lang::{Lang, Types};

pub struct GolangExporter;

impl GolangExporter {
    pub fn default() -> Self {
        GolangExporter
    }
}

impl LangExporterBuilderCustomizer for GolangExporter {
    fn apply(&self, builder: &mut LangExporterBuilder) {
        builder.add_modifier(|ctx, data| {
            let name = ctx.tb.name.replace(".xlsx", "");

            data.pkg = ctx.args.pkg.clone().to_lowercase();
            data.filename = to_lowercase(ctx.args.lang_export.gen_lang_name(
                name.as_str(),
                ctx.tb.sheet_name.as_str(),
                &ctx.args.lang).as_str());
            data.name = to_first_uppercase(name.as_str());
            // data.imports.push("import \"github.com/abcd\"".to_string());
            ctx.tb.header.iter().for_each(|h| {
                data.fields.push(LangFieldData {
                    field_name: to_first_uppercase(h.field_name.as_str()),
                    field_type: to_go_type(h.field_type.as_str()),
                    field_comment: h.comment.clone(),
                });
            });
        });
    }
}

fn to_lowercase(src: &str) -> String {
    let string = src.to_string().chars()
        .map(|c| {
            if c.is_uppercase() {
                return format!("_{}", c.to_lowercase());
            }
            c.to_string()
        })
        .fold("".to_string(), |c1, c2| format!("{}{}", c1, c2));
    if string.starts_with("_") {
        return string[1..].to_string();
    }
    string
}

fn to_first_uppercase(src: &str) -> String {
    let pos = 0;
    format!("{}{}", src[..pos].to_uppercase(), src[pos + 1..].to_string())
}

fn to_go_type(t: &str) -> String {
    let ret = match Types::from_str(t) {
        Ok(typ) => match typ {
            Types::String => "String",
            Types::Strings => "[]String",
            Types::Int => "int32",
            Types::Ints => "[]int32",
            Types::Double => "float64",
            Types::Doubles => "[]float64",
            Types::Byte => "byte",
            Types::Bytes => "[]byte",
            Types::Bool => "bool",
            Types::Bools => "[]bool",
            Types::Json => "String",
            Types::Float => "float32",
            Types::Floats => "[]float32",
        }
        Err(_) => { "" }
    };
    ret.to_string()
}