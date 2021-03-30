use crate::lib::PageStyler;
use clap::{App, Arg, SubCommand};
use log::info;
use mdbook::preprocess::Preprocessor;

mod lib;

pub fn make_app() -> App<'static, 'static> {
    App::new("page-style-preprocessor")
        .about("A mdbook preprocessor which applies the desired styles to individual pages and elements")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    env_logger::init();
    info!("Starting...");
    let matches = make_app().get_matches();

    let preprocessor = PageStyler::new();

    info!("matches: {:?}", matches);
    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args
            .value_of("renderer")
            .expect("`supports` subcommand requires a renderer argument");
        let is_supported = preprocessor.supports_renderer(&renderer);
        info!("Is supported? {}", is_supported);
        match is_supported {
            true => std::process::exit(0),
            false => std::process::exit(1),
        }
    } else {
        info!("Starting preprocessor");
        handle_preprocessing(&preprocessor).expect("Problem processing book");
    }
}

use mdbook::errors::Error;
use mdbook::preprocess::CmdPreprocessor;

fn handle_preprocessing(preprocessor: &dyn Preprocessor) -> Result<(), Error> {
    info!("Parsing Input");
    let (ctx, book) = CmdPreprocessor::parse_input(std::io::stdin())?;

    let processed_book = preprocessor.run(&ctx, book)?;

    serde_json::to_writer(std::io::stdout(), &processed_book)?;
    Ok(())
}
