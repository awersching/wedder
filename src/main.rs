#[macro_use]
extern crate strum_macros;

use std::process;

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

fn handle_args(args: &CmdArgs) {
    if args.debug {
        TermLogger::init(LevelFilter::Info, simplelog::Config::default())
            .unwrap();
    }

    if args.print_default_config_path {
        println!("{}", Config::default_config_path().to_str().unwrap());
        process::exit(0);
    }
}

fn create_config(args: CmdArgs) -> Config {
    let mut config = match &args.config_file {
        Some(path) => Config::from_path(path),
        None => Config::from_default_path()
    };
    config.merge(args);

    if config.weather.api_key == "" {
        println!("No API key");
        process::exit(1)
    }
    config
}

fn main() {
    let args = CmdArgs::from_args();
    handle_args(&args);
    let config = create_config(args);

    let app = App::new(config);
    if let Err(err) = app.run() {
        error!("{:?}", err)
    }
}
