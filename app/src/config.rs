/// Settings that decide how requests are accepted.
#[derive(Clone)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    /// Browser origins allowed to call the API. Empty means same origin only.
    pub cors_origins: Vec<String>,
    /// Requests per minute and client address on `/auth`. 0 turns the limit off.
    pub auth_rate_limit: u32,
    /// Swagger UI and the OpenAPI file, served only when `APP_ENV=development`.
    pub docs_enabled: bool,
}

impl SecurityConfig {
    pub fn from_env() -> SecurityConfig {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    fn from_lookup(get: impl Fn(&str) -> Option<String>) -> SecurityConfig {
        let jwt_secret = get("JWT_SECRET").expect("JWT_SECRET is not set in .env file");
        assert!(
            jwt_secret.len() >= 32,
            "JWT_SECRET must be at least 32 characters"
        );
        let non_empty = |name: &str| get(name).filter(|value| !value.trim().is_empty());

        SecurityConfig {
            jwt_secret,
            jwt_issuer: non_empty("JWT_ISSUER").unwrap_or_else(|| "{{project-name}}".to_string()),
            jwt_audience: non_empty("JWT_AUDIENCE")
                .unwrap_or_else(|| "{{project-name}}-client".to_string()),
            cors_origins: get("CORS_ORIGINS")
                .unwrap_or_default()
                .split(',')
                .map(|origin| origin.trim().to_string())
                .filter(|origin| !origin.is_empty())
                .collect(),
            auth_rate_limit: non_empty("AUTH_RATE_LIMIT").map_or(10, |value| {
                value
                    .trim()
                    .parse()
                    .expect("AUTH_RATE_LIMIT is not a number")
            }),
            // anything else, including nothing, is treated as production
            docs_enabled: get("APP_ENV").is_some_and(|value| value == "development"),
        }
    }
}

pub struct Config {
    pub db_url: String,
    pub host: String,
    pub port: u32,
    pub prefork: bool,
    pub security: SecurityConfig,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file"),
            host: std::env::var("HOST").expect("HOST is not set in .env file"),
            port: std::env::var("PORT")
                .expect("PORT is not set in .env file")
                .parse()
                .expect("PORT is not a number"),
            prefork: std::env::var("PREFORK").is_ok_and(|v| v == "1"),
            security: SecurityConfig::from_env(),
        }
    }

    pub fn get_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::SecurityConfig;

    fn config(vars: &[(&str, &str)]) -> SecurityConfig {
        SecurityConfig::from_lookup(|name| {
            if name == "JWT_SECRET" {
                return Some("0123456789abcdef0123456789abcdef".to_string());
            }
            vars.iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| value.to_string())
        })
    }

    #[test]
    fn defaults_are_the_safe_ones() {
        let config = config(&[]);
        assert!(!config.docs_enabled);
        assert!(config.cors_origins.is_empty());
        assert_eq!(config.auth_rate_limit, 10);
    }

    #[test]
    fn reads_the_environment() {
        let config = config(&[
            ("APP_ENV", "development"),
            ("CORS_ORIGINS", "http://a.test, http://b.test,"),
            ("AUTH_RATE_LIMIT", "0"),
        ]);
        assert!(config.docs_enabled);
        assert_eq!(config.cors_origins, ["http://a.test", "http://b.test"]);
        assert_eq!(config.auth_rate_limit, 0);
    }
}
