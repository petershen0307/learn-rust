pub struct Configuration {
    pub port: i32,
    pub workers: usize,
}

impl Configuration {
    pub fn new() -> Self {
        const DEFAULT_PORT: i32 = 6379;
        const DEFAULT_WORKERS: usize = 1;
        let port = std::env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
        let workers = std::env::var("WORKERS").unwrap_or(DEFAULT_WORKERS.to_string());
        Configuration {
            port: port.parse::<i32>().unwrap_or(DEFAULT_PORT),
            workers: workers.parse::<usize>().unwrap_or(DEFAULT_WORKERS),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
