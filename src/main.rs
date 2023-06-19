use tera::{Context, Tera};

use crate::args::{BuildArgs, BuildArgsBuilder, BuildMode, ConfigExportFileType, ExportArgs, ExtractArgsBuilder, LinePattern, LinePatternBuilder, LoadMode};
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

    // cfg loader 目前只支持xlsx
    let loader = XlsxConfigLoader::new();
    // build options
    let args_builder = BuildArgsBuilder::new()
        .set_build_mode(BuildMode::Server)
        .set_config_path("config".to_string())
        .set_lang(Lang::Go)
        .set_config_load_mode(LoadMode::AllSheets)
        .set_pkg("cfg".to_string())
        .set_config_comment_pattern(LinePatternBuilder::new()
            .set_name("comment".to_string())
            .set_line_no(0)
            .set_extractor(BuildArgs::default_lp_extractor)
            .build())
        .set_config_name_pattern(LinePatternBuilder::new()
            .set_name("name".to_string())
            .set_line_no(1)
            .set_extractor(BuildArgs::default_lp_extractor)
            .build())
        .set_config_type_pattern(LinePatternBuilder::new()
            .set_name("type".to_string())
            .set_line_no(1)
            .set_extractor(BuildArgs::default_lp_extractor)
            .build())
        .set_config_mode_pattern(LinePatternBuilder::new()
            .set_name("mode".to_string())
            .set_line_no(3)
            .set_extractor(BuildArgs::default_lp_extractor)
            .build())
        .set_config_file_type(ConfigExportFileType::Csv)
        .set_lang_export(ExtractArgsBuilder::new()
            .set_out_dir("gen/src".to_string())
            .set_naming_func(BuildArgs::default_naming_func)
            .build())
        .set_config_export(ExtractArgsBuilder::new()
            .set_out_dir("gen/cfg".to_string())
            .set_naming_func(BuildArgs::default_naming_func)
            .build());
    let args = args_builder.build();

    // config exporter
    let cfg_exp = CsvExporter::new();

    // lang exporter
    let mut lang_exp = LangExporterBuilder::new()
        .add_modifier(|ctx, data| {
            let name = ctx.tb.name.replace(".xlsx", "");

            data.pkg = ctx.args.pkg.clone();
            data.filename = ctx.args.lang_export.gen_lang_name(
                name.as_str(),
                ctx.tb.sheet_name.as_str(),
                &ctx.args.lang);
            data.name = name.clone();
            // data.imports.push("import \"github.com/abcd\"".to_string());
            ctx.tb.header.iter().for_each(|h| {
                data.fields.push(LangFieldData {
                    field_name: h.field_name.clone(),
                    field_type: h.field_type.clone(),
                    field_comment: h.comment.clone(),
                });
            });
        }).build();//BaseLangLifetime::wrap(Box::new(GolangExporter::new()));

    loader.load(&args, |ctx| {
        println!("load table: {}/{}", ctx.tb.name, ctx.tb.sheet_name);
        cfg_exp.export(ctx);
        lang_exp.export(ctx);
    });
    
    println!("load complete");
}

