# C Portal API

1. Modify the `DATABASE_URL` var in `.env` to point to your chosen database

1. Turn on the appropriate database feature for your chosen db in `service/Cargo.toml` (the `"sqlx-postgres",` line)

1. Execute `cargo run` to start the server

1. Visit [localhost:8000](http://localhost:8000) in browser

Run mock test on the service logic crate:

```bash
cd service
cargo test --features mock
```

## Auto Reload

```sh
cargo install cargo-watch systemfd
```

## Running

```sh
systemfd --no-pid -s http::50083 -- cargo watch -x run
```