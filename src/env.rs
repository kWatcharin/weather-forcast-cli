use std::env;
use std::sync::{Arc, RwLock};
use dotenv::{dotenv, from_path};
use lazy_static::lazy_static;


pub mod base_env {
    use super::*;

    pub fn load_env() {
        from_path("./.env").ok();
    }

    lazy_static! {
        #[derive(Debug)]
        pub static ref API_KEY: Arc<RwLock<String>> = Arc::new(
            RwLock::new(
                env::var("API_KEY").expect("API_KEY must be set!")
            )
        );
    }
}


