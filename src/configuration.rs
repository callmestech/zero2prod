use config::Config;

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()
        .unwrap();
    settings.try_deserialize()
}
