use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rload")]
#[command(about = "Minimal Rust load testing tool", long_about = None)]
pub struct CliArgs {
    /// Target URL
    #[arg(long)]
    pub url: String,

    /// HTTP method (GET, POST)
    #[arg(long, default_value = "GET")]
    pub method: String,

    /// Total number of requests
    #[arg(long)]
    pub requests: u64,

    /// Number of concurrent workers
    #[arg(long, default_value = "10")]
    pub concurrency: usize,

    /// Headers (can be repeated)
    #[arg(long)]
    pub header: Vec<String>,

    /// Request body (for POST)
    #[arg(long)]
    pub body: Option<String>,
}
