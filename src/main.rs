mod cli;
mod config;

use clap::Parser;
use cli::CliArgs;
use config::Config;

fn main() {
    let args = CliArgs::parse();
    let config = Config::from_cli(args);

    println!("Parsed Config:");
    println!("URL: {}", config.url);
    println!("Method: {}", config.method);
    println!("Requests: {}", config.total_requests);
    println!("Concurrency: {}", config.concurrency);

    println!("Headers:");
    for (k, v) in config.headers {
        println!("  {}: {}", k, v);
    }

    if let Some(body) = config.body {
        println!("Body: {}", body);
    }
}