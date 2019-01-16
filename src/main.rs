#[macro_use]
extern crate strum_macros;

use std::process;

use log::debug;
use log::error;
use simplelog::LevelFilter;
use simplelog::TermLogger;
use structopt::StructOpt;

use crate::app::App;
use crate::config::cmd_args::CmdArgs;
use crate::config::Config;

mod config;
mod weather;
mod util;
mod location;
mod app;

fn main() {
    let args = CmdArgs::from_args();
    handle_args(&args);
    let config = create_config(args);

    let app = App::new(config);
    if let Err(err) = app.run() {
        error!("{}", err.to_string())
    }
}

fn handle_args(args: &CmdArgs) {
    if args.debug {
        init_logger();
        debug!("Read args {:?}", args);
    }

    if args.print_default_config_path {
        println!("{}", Config::default_config_path().unwrap().to_str().unwrap());
        process::exit(0);
    }
}

fn create_config(args: CmdArgs) -> Config {
    let mut config = match &args.config_file {
        Some(path) => Config::from_path(path),
        None => Config::from_default_path()
    };
    debug!("Read config {:?}", config);
    config.merge(args);
    debug!("Merged config with args into {:?}", config);

    if config.weather.api_key == "" {
        println!("No API key");
        process::exit(1)
    }
    config
}

fn init_logger() {
    TermLogger::init(LevelFilter::Debug, simplelog::Config::default())
        .unwrap();
}
