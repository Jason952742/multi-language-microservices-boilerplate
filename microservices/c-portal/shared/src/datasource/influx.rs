use std::env;
use chrono::{DateTime, Utc};
use colored::Colorize;
use influxdb::{Client, InfluxDbWriteable};
use tokio::sync::OnceCell;
use tracing::info;


#[derive(Debug)]
pub struct InfluxPool;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

impl InfluxPool {
    pub async fn connection() -> &'static Client {
        CLIENT
            .get_or_init(|| async {
                dotenvy::dotenv().ok();
                let uri = env::var("INFLUXDB_URI").expect("INFLUXDB_URI must be set");
                let bucket = env::var("INFLUXDB_BUCKET").expect("INFLUXDB_BUCKET must be set");
                let token = env::var("INFLUXDB_BUCKET_TOKEN").expect("INFLUXDB_BUCKET_TOKEN must be set");
                let client = Client::new(uri, bucket).with_token(token);
                info!("{}", "INFLUX CONNECTED".color("magenta"));
                client
            }).await
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use influxdb::{InfluxDbWriteable, ReadQuery, Timestamp};

    let client = InfluxPool::connection().await;
    // Let's write some data into a measurement called `weather`
    let weather_readings = vec![
        WeatherReading { time: Timestamp::Hours(1).into(), humidity: 30, wind_direction: String::from("north") }.into_query("weather"),
        WeatherReading { time: Timestamp::Hours(2).into(), humidity: 40, wind_direction: String::from("west") }.into_query("weather"),
    ];

    client.query(weather_readings).await?;

    // Let's see if the data we wrote is there
    let read_query = ReadQuery::new("SELECT * FROM weather");

    let read_result = client.query(read_query).await?;
    println!("{}", read_result);

    Ok(())
}

#[derive(InfluxDbWriteable)]
struct WeatherReading {
    time: DateTime<Utc>,
    humidity: i32,
    #[influxdb(tag)]
    wind_direction: String,
}
