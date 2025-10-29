#[cfg(feature = "auth")]
pub mod auth {
    pub fn init() {
        println!("Auth module initialized");
    }
}

#[cfg(feature = "storage")]
pub mod storage {
    pub fn init() {
        println!("Storage module initialized");
    }
}

#[cfg(feature = "database")]
pub mod database {
    pub fn init() {
        println!("Database module initialized");
    }
}

pub fn init() {
    #[cfg(feature = "auth")]
    auth::init();

    #[cfg(feature = "storage")]
    storage::init();

    #[cfg(feature = "database")]
    database::init();
}