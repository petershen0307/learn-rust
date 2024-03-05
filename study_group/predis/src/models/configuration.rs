pub struct Configuration {
    pub port: i32,
    pub workers: i32,
}

impl Configuration {
    pub fn new() -> Self {
        const DEFAULT_PORT: i32 = 6379;
        const DEFAULT_WORKERS: i32 = 1;
        let port = std::env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
        let workers = std::env::var("WORKERS").unwrap_or(DEFAULT_WORKERS.to_string());
        Configuration {
            port: port.parse::<i32>().unwrap_or(DEFAULT_PORT),
            workers: workers.parse::<i32>().unwrap_or(DEFAULT_WORKERS),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}
