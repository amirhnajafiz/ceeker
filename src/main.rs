use config::Config;
use serde::Deserialize;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct Settings {
    host: String,
    port: u16,
}

fn main() {
    dotenv().ok(); // Loads from .env file into std::env

    let settings = Config::builder()
        .add_source(config::Environment::default()) // reads from env vars
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    println!("Host: {}, Port: {}", settings.host, settings.port);
}

