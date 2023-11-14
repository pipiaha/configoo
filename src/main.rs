use clap::builder::Str;
use clap::Parser;
use tera::{Context, Tera};

use crate::args::{BuildArgs, BuildArgsBuilder, BuildMode, ConfigExportFileType, ExportArgs, ExtractArgsBuilder, LinePattern, LinePatternBuilder, LoadMode};
use crate::context::func::{ConfigExporter, ConfigLoader, LangExporter, LangExporterBuilder, LangExporterBuilderCustomizer, LangTemplateDataModifier};
use crate::context::model::{LangFieldData, LangTemplateData};
use crate::export::config::CsvExporter;
use crate::export::lang_go::GolangExporter;
use crate::import::importer::XlsxConfigLoader;
use crate::lang::Lang;

mod context;
mod export;
mod import;
mod lang;
mod args;
mod embed;

fn main() {
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    // cfg loader 目前只支持xlsx
    let loader = XlsxConfigLoader::new();
    // build options
    let args_builder = BuildArgsBuilder::new()
        .set_build_mode(BuildMode::Server)
        .set_config_path("config".to_string())
        .set_lang(Lang::Go)
        .set_config_load_mode(LoadMode::AllSheets)
        .set_pkg("cfgs".to_string())
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
            .set_line_no(2)
            .set_extractor(BuildArgs::default_lp_extractor)
            .build())
        .set_config_mode_pattern(LinePatternBuilder::new()
            .set_name("mode".to_string())
            .set_line_no(3)
            .set_extractor(BuildArgs::default_lp_extractor)
            .build())
        .set_config_file_type(ConfigExportFileType::Csv)
        .set_lang_export(ExtractArgsBuilder::new()
            .set_out_dir("gen/src/go".to_string())
            .set_naming_func(BuildArgs::default_naming_func)
            .build())
        .set_config_export(ExtractArgsBuilder::new()
            .set_out_dir("gen/cfg/csv".to_string())
            .set_naming_func(BuildArgs::default_naming_func)
            .build());
    let args = args_builder.build();

    // config exporter
    let cfg_exp = CsvExporter::new();

    // lang exporter
    let mut builder = LangExporterBuilder::new();
    GolangExporter::default().apply(&mut builder);
    let mut lang_exp = builder.build();

    loader.load(&args, |ctx| {
        println!("load table: {}/{}", ctx.tb.name, ctx.tb.sheet_name);
        cfg_exp.export(ctx);
        lang_exp.export(ctx);
    });

    println!("load complete");
}


#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short = 'b', long)]
    build: Option<BuildMode>,
    #[arg(short = 'p', long)]
    path: Option<String>,
    #[arg(short = 'l', long)]
    load: Option<LoadMode>,
    #[arg(long)]
    comment_line_no: Option<i32>,
    #[arg(long)]
    type_line_no: Option<i32>,
    #[arg(long)]
    name_line_no: Option<i32>,
    #[arg(long)]
    mode_line_no: Option<i32>,
    #[arg(long)]
    out_config_dir: Option<String>,
    #[arg(long)]
    out_config_type: Option<ConfigExportFileType>,
    #[arg(long)]
    out_config_naming: Option<String>,
    #[arg(long)]
    out_lang_type: Option<Lang>,
    #[arg(long)]
    out_lang_dir: Option<String>,
    #[arg(long)]
    out_lang_naming: Option<String>,
    #[arg(long)]
    out_lang_pkg: Option<String>,
}