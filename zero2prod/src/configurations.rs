use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder();
    let base_path = std::env::current_dir().expect("Faile to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let settings = settings
        .add_source(config::File::from(configuration_directory.join("base")).required(true));

    let environement: Environement = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONEMENT.");

    let settings = settings
        .add_source(
            config::File::from(configuration_directory.join(environement.as_str())).required(true),
        )
        .build()?;

    settings.try_deserialize()
}

pub enum Environement {
    Local,
    Production,
}

impl Environement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environement::Local => "local",
            Environement::Production => "production",
        }
    }
}

impl TryFrom<String> for Environement {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environement. Use either `local` or `production`.",
                other
            )),
        }
    }
}
