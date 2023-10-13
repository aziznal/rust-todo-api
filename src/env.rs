use dotenv;
use envy;
use lazy_static::lazy_static;
use serde::Deserialize;

use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Environment {
    /** The host where this server is running */
    pub host: String,

    /** The port number where this server is running */
    pub port: i32,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

lazy_static! {
    pub static ref ENV: Environment = load_env();
}

fn load_env() -> Environment {
    println!("Loading environment variables from .env");
    // check env file exists
    dotenv::dotenv().expect("Expected .env file in root directory");

    // read env
    let env = envy::from_env::<Environment>()
        .unwrap_or_else(|e| panic!("Failed to read environment, {}", e));

    println!("Successfully loaded environment variables from .env");

    return env;
}
