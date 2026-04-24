pub struct Config {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub concurrency: usize,
    pub total_requests: u64,
}

impl Config {
    pub fn from_cli(args: crate::cli::CliArgs) -> Self {
        let headers = args
            .header
            .into_iter()
            .filter_map(|h| {
                let parts: Vec<&str> = h.splitn(2, ':').collect();
                if parts.len() == 2 {
                    Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                } else {
                    None
                }
            })
            .collect();

        Self {
            url: args.url,
            method: args.method.to_uppercase(),
            headers,
            body: args.body,
            concurrency: args.concurrency,
            total_requests: args.requests,
        }
    }
}