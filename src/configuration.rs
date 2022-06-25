use config::Config;
use secrecy::{Secret, ExposeSecret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings{
    pub port: u16,
    pub host:String,
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
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}

pub enum Environment{
    Local,
    Production,
}
impl Environment{
   pub fn as_str(&self) -> &'static str {
       match self {
           Environment::Local => "local",
           Environment::Production => "production"
       }
   }
}
impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s:String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str(){
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. User local or production.", other
            ))
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // let mut settings = config::Config::default();
    // settings.merge(config::File::with_name("configuration"))?;
    // settings.try_into()
    let base_path=std::env::current_dir().expect("Unable to determine current directory");
    let configuration_directory = base_path.join("configuration");
let environment:Environment = std::env::var("APP_ENVIRONMENT")
    .unwrap_or_else(|_| "local".into())
    .try_into()
    .expect("Failed to parse APP_ENVIRONMENT.")
    ;

    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::from(
            configuration_directory.join("base")
        ).required(true))
        .add_source(config::File::from(
            configuration_directory.join(environment.as_str())
        ).required(true))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    settings.try_deserialize::<Settings>()
}
