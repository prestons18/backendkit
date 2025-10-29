use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct BackendKitConfig {
    name: String,
    version: String,
    features: Features,
    auth: Option<AuthConfig>,
    storage: Option<StorageConfig>,
    database: Option<DatabaseConfig>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Features {
    auth: bool,
    storage: bool,
    database: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct AuthConfig {
    adapter: String,
    secret: String,
    expiry: u64,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct StorageConfig {
    adapter: String,
    bucket: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DatabaseConfig {
    adapter: String,
    url: String,
}