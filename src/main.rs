use std::error::Error;
use std::process;

use structopt::StructOpt;

use crate::app::App;
use crate::config::cli_args::CliArgs;
use crate::config::Config;

mod config;
mod weather;
mod location;
mod app;
mod http;
mod logger;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() {
    if let Err(err) = run() {
        println!("Error: {}", err.to_string());
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = CliArgs::from_args();
    args.apply()?;
    let config = Config::new(args);

    let app = App::new(config);
    app.run()
}
