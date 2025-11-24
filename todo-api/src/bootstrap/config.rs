use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cors: CorsConfig,
    pub logging: LoggingConfig,
    // pub jwt: JwtConfig,
    // pub oauth: OauthConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl DatabaseConfig {
    pub fn to_postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allow_origin: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

// #[derive(Debug, Deserialize, Clone)]
// pub struct JwtConfig {
//     pub access_secret: String,
//     pub refresh_secret: String,
//     pub access_expire_seconds: u64,
//     pub refresh_expire_seconds: u64,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct OauthConfig {
//     pub kakao_client_id: String,
//     pub kakao_client_secret: String,
//     pub redirect_uri: String,
// }

pub fn load_config() -> AppConfig {
    dotenvy::dotenv().ok();

    let builder = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name("config/local").required(false))
        .add_source(Environment::with_prefix("APP").separator("__"));

    builder.build().unwrap().try_deserialize().unwrap()
}
