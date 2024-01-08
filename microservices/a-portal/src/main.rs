use std::env;
use clap::Parser;
use colored::Colorize;
use tracing::info;
use api::start;
use shared::{Config, consul_api};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set log level
    env::set_var("RUST_LOG", "debug");

    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();


    // register consul service
    consul_register(&config.host, &config.port).await;

    let result = api::start(config).await;

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }

    Ok(())
}

// register consul service
async fn consul_register(host: &str, port: &i32) {
    let cs = consul_api::Consul::new(consul_api::ConsulOption::default()).unwrap();
    let reg = consul_api::Registration::simple(consul_api::ServiceName::MuAPortal, host, *port, false);
    cs.register(&reg).await.unwrap();
    info!("{} Successfully Registered", consul_api::ServiceName::MuAPortal.to_string().color("cyan"));
    tokio::spawn(async move {
        cs.discover_service().await.expect("discover_service failed");
    });
}
