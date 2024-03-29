// We prefer to keep `main.rs` and `lib.rs` separate as it makes it easier to add extra helper
// binaries later which share code with the main project. It could save you from a nontrivial
// refactoring effort in the future.
//
// Whether to make `main.rs` just a thin shim that awaits a `run()` function in `lib.rs`, or
// to put the application bootstrap logic here is an open question. Both approaches have their
// upsides and their downsides. Your input is welcome!

use std::env;
use clap::Parser;
use tracing::info;
use colored::Colorize;
use shared::{config::Config, consul_api};

#[tokio::main]
async  fn main() -> anyhow::Result<()> {
    // set log level
    env::set_var("RUST_LOG", "debug");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_test_writer()
        .init();

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
    let reg = consul_api::Registration::simple(consul_api::ServiceName::MuMember, host, *port, true);
    cs.register(&reg).await.unwrap();
    info!("{} Successfully Registered", consul_api::ServiceName::MuMember.to_string().color("cyan"));
    tokio::spawn(async move {
        cs.discover_service().await.expect("discover_service failed");
    });
}
