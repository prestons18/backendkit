use serde::Deserialize;

#[derive(Deserialize)]
pub struct BackendKitConfig {
    pub name: String,
    pub version: String,
    pub features: Features,
    pub auth: Option<AuthConfig>,
    pub storage: Option<StorageConfig>,
    pub database: Option<DatabaseConfig>,
}

#[derive(Deserialize)]
pub struct Features {
    pub auth: bool,
    pub storage: bool,
    pub database: bool,
}

#[derive(Deserialize)]
pub struct AuthConfig {
    pub adapter: String,
    pub secret: String,
    pub expiry: u64,
}

#[derive(Deserialize)]
pub struct StorageConfig {
    pub adapter: String,
    pub bucket: String,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub adapter: String,
    pub url: String,
}