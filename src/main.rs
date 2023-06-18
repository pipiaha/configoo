use tera::{Context, Tera};

use crate::args::{BuildArgs, BuildMode, ExportArgs, LinePattern, LoadMode};
use crate::context::func::{ConfigExporter, ConfigLoader, LangExporter, LangExporterBuilder, LangTemplateDataModifier};
use crate::context::model::{LangFieldData, LangTemplateData};
use crate::export::config::CsvExporter;
use crate::import::importer::XlsxConfigLoader;
use crate::lang::Lang;

mod context;
mod export;
mod import;
mod lang;
mod args;
mod embed;

fn main() {
    // let args = env::args();
    // let mut excel = Excel::open("config/file.xlsx").unwrap();
    // let r = excel.worksheet_range("Sheet1").unwrap();
    // for row in r.rows() {
    //     println!("row={:?}, row[0]={:?}", row, row[0]);
    // }
    let dir = "config";
    //
    // let args = BuildArgs {};

    // config exporter
    let cfg_exp = CsvExporter::new();
    // TODO go 源文件设置
    let mut lang_exp = LangExporterBuilder::new()
        .add_modifier(|ctx, data| {
            data.pkg = ctx.args.pkg.clone();
            data.name = ctx.args.lang_export.naming.gen_lang_name(ctx.tb.name.as_str(), ctx.tb.sheet_name.as_str(), &ctx.args.lang);
            data.filename = ctx.tb.name.clone();
            // data.imports.push("import \"github.com/abcd\"".to_string());
            ctx.tb.header.iter().for_each(|h| {
                data.fields.push(LangFieldData {
                    field_name: h.field_name.clone(),
                    field_type: h.field_type.clone(),
                    field_comment: h.comment.clone(),
                });
            });
        }).build();//BaseLangLifetime::wrap(Box::new(GolangExporter::new()));

    let loader = XlsxConfigLoader::new();
    let args = BuildArgs {
        mode: BuildMode::Server,
        path: "config".to_string(),
        lang: Lang::Go,
        load: LoadMode::AllSheets,
        pkg: "cfg".to_string(),
        comment_pattern: LinePattern {
            name: "comment".to_string(),
            line_no: 0,
            extractor: |s| s.to_string(),// TODO 预定义
        },
        type_pattern: LinePattern {
            name: "type".to_string(),
            line_no: 2,
            extractor: |s| s.to_string(),
        },
        name_pattern: LinePattern {
            name: "name".to_string(),
            line_no: 1,
            extractor: |s| s.to_string(),
        },
        mode_pattern: LinePattern {
            name: "mode".to_string(),
            line_no: 3,
            extractor: |s| s.to_string(),
        },
        config_export: ExportArgs { out_dir: "gen/cfg".to_string(), naming: Default::default() },
        lang_export: ExportArgs { out_dir: "gen/src".to_string(), naming: Default::default() },
    };
    loader.load(&args, dir, |ctx| {
        println!("load table: {}-{}", ctx.tb.name, ctx.tb.sheet_name);
        cfg_exp.export(ctx);
        lang_exp.export(ctx);
    });

    println!("load complete");
}

