pub struct Config {
    pub db_url: String,
    pub host: String,
    pub port: u32,
    pub prefork: bool,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Config {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file");
        assert!(
            jwt_secret.len() >= 32,
            "JWT_SECRET must be at least 32 characters"
        );

        Config {
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"),
            host: std::env::var("HOST").expect("HOST is not set in .env file"),
            port: std::env::var("PORT")
                .expect("PORT is not set in .env file")
                .parse()
                .expect("PORT is not a number"),
            prefork: std::env::var("PREFORK").is_ok_and(|v| v == "1"),
            jwt_secret,
        }
    }

    pub fn get_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
