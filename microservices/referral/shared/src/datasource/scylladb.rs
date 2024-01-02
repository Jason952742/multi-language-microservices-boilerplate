use std::env;
use scylla::{Session, SessionBuilder};
use tokio::sync::OnceCell;
use tracing::info;
use colored::Colorize;
use scylla::transport::errors::QueryError;

pub struct ScyllaPool;

static SESSION: OnceCell<Session> = OnceCell::const_new();

impl ScyllaPool {
    pub async fn connection() -> &'static Session {
        SESSION
            .get_or_init(|| async {
                dotenvy::dotenv().ok();
                let uri = env::var("SCYLLA_URI").expect("SCYLLA_URI must be set");
                let user = env::var("SCYLLA_USER").expect("SCYLLA_USER must be set");
                let password = env::var("SCYLLA_PASSWORD").expect("SCYLLA_PASSWORD must be set");

                let session: Session = SessionBuilder::new()
                    .known_node(uri)
                    .user(user, password)
                    .build()
                    .await.expect("Scylladb connection failed");
                info!("{}", "SCYLLA CONNECTED".color("magenta"));
                session
            })
            .await
    }

    pub async fn init_keyspace(session: &Session, keyspace: &str, factor: u16) -> Result<String, QueryError> {
        let query = format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class' : 'NetworkTopologyStrategy', 'replication_factor' : {}}}", keyspace, factor);
        let _ = session.query(query, &[]).await?;
        Ok(keyspace.to_string())
    }

    pub async fn init_table(session: &Session, keyspace: &str, table: &str, column: &str) -> Result<String, QueryError> {
        let q = format!("CREATE TABLE IF NOT EXISTS {}.{} ({})", keyspace, table, column);
        let _ = session.query(q, &[]).await?;
        Ok(table.to_string())
    }

    pub async fn init_type(session: &Session, keyspace: &str, type_name: &str, fields: &str) -> Result<String, QueryError> {
        let q = format!("CREATE TYPE IF NOT EXISTS {}.{} ({})", keyspace, type_name, fields);
        let _ = session.query(q, &[]).await?;
        Ok(type_name.to_string())
    }

}


#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use scylla::{FromRow, FromUserType, impl_from_cql_value_from_method, IntoTypedRows, SerializeCql, Session, SessionBuilder};

    let session = ScyllaPool::connection().await;
    let keyspace = ScyllaPool::init_keyspace(session, "ks", 1).await?;
    ScyllaPool::init_table(session, &keyspace, "t", "a int, b int, c text, primary key (a, b)").await?;

    session.query("INSERT INTO ks.t (a, b, c) VALUES (?, ?, ?)", (3, 4, "def")).await?;
    session.query("INSERT INTO ks.t (a, b, c) VALUES (1, 2, 'abc')", &[]).await?;

    // let prepared = session.prepare("INSERT INTO ks.t (a, b, c) VALUES (?, 7, ?)").await?;
    // session.execute(&prepared, (42_i32, "I'm prepared!")).await?;
    // session.execute(&prepared, (43_i32, "I'm prepared 2!")).await?;
    // session.execute(&prepared, (44_i32, "I'm prepared 3!")).await?;

    // Rows can be parsed as tuples
    // if let Some(rows) = session.query("SELECT a, b, c FROM ks.t", &[]).await?.rows {
    //     for row in rows.into_typed::<(i32, i32, String)>() {
    //         let (a, b, c) = row?;
    //         println!("a, b, c: {}, {}, {}", a, b, c);
    //     }
    // }

    // Or as custom structs that derive FromRow
    #[derive(Debug, FromRow)]
    struct RowData {
        _a: i32,
        _b: Option<i32>,
        _c: String,
    }

    if let Some(rows) = session.query("SELECT a, b, c FROM ks.t", &[]).await?.rows {
        for row_data in rows.into_typed::<RowData>() {
            let row_data = row_data?;
            println!("row_data: {:?}", row_data);
        }
    }

    // Or simply as untyped rows
    // if let Some(rows) = session.query("SELECT a, b, c FROM ks.t", &[]).await?.rows {
    //     for row in rows {
    //         let a = row.columns[0].as_ref().unwrap().as_int().unwrap();
    //         let b = row.columns[1].as_ref().unwrap().as_int().unwrap();
    //         let c = row.columns[2].as_ref().unwrap().as_text().unwrap();
    //         println!("a, b, c: {}, {}, {}", a, b, c);
    //
    //         // Alternatively each row can be parsed individually
    //         // let (a2, b2, c2) = row.into_typed::<(i32, i32, String)>() ?;
    //     }
    // }

    ScyllaPool::init_type(session, &keyspace, "my_type", "int_val int, text_val text").await?;
    ScyllaPool::init_table(session, &keyspace, "udt_tab", "k int, my my_type, primary key (k)").await?;

    // Define custom struct that matches User Defined Type created earlier
    // wrapping field in Option will gracefully handle null field values
    #[derive(Debug, FromUserType, SerializeCql)]
    struct MyType {
        int_val: i32,
        text_val: Option<String>,
    }

    let to_insert = MyType { int_val: 17, text_val: Some("Some string".to_string()) };

    // It can be inserted like a normal value
    session.query("INSERT INTO ks.udt_tab (k, my) VALUES (5, ?)", (to_insert,)).await?;

    // And read like any normal value
    if let Some(rows) = session.query("SELECT my FROM ks.udt_tab", &[]).await?.rows {
        for row in rows.into_typed::<(MyType,)>() {
            let (my_type_value,): (MyType,) = row?;
            println!("{:?}", my_type_value)
        }
    }


    let metrics = session.get_metrics();
    println!("Queries requested: {}", metrics.get_queries_num());
    println!("Iter queries requested: {}", metrics.get_queries_iter_num());
    println!("Errors occurred: {}", metrics.get_errors_num());
    println!("Iter errors occurred: {}", metrics.get_errors_iter_num());
    println!("Average latency: {}", metrics.get_latency_avg_ms().unwrap());
    println!(
        "99.9 latency percentile: {}",
        metrics.get_latency_percentile_ms(99.9).unwrap()
    );

    println!("Ok.");


    Ok(())
}

#[tokio::test]
async fn select_paging() -> Result<(), Box<dyn std::error::Error>> {
    use futures::stream::StreamExt;
    use scylla::{query::Query};

    let session = ScyllaPool::connection().await;
    let keyspace = ScyllaPool::init_keyspace(session, "ks", 1).await?;
    let table = ScyllaPool::init_table(session, &keyspace, "t", "a int, b int, c text, primary key (a, b)").await?;

    for i in 0..16_i32 {
        session.query(format!("INSERT INTO {}.{} (a, b, c) VALUES (?, ?, 'abc')", &keyspace, &table), (i, 2 * i)).await?;
    }

    // Iterate through select result with paging
    let mut rows_stream = session.query_iter("SELECT a, b, c FROM ks.t", &[]).await?.into_typed::<(i32, i32, String)>();

    while let Some(next_row_res) = rows_stream.next().await {
        let (a, b, c) = next_row_res?;
        println!("a, b, c: {}, {}, {}", a, b, c);
    }

    let paged_query = Query::new("SELECT a, b, c FROM ks.t").with_page_size(6);

    let res1 = session.query(paged_query.clone(), &[]).await?;
    println!("Paging state: {:#?} ({} rows)", res1.paging_state, res1.rows.unwrap().len());

    let res2 = session.query_paged(paged_query.clone(), &[], res1.paging_state).await?;
    println!("Paging state: {:#?} ({} rows)", res2.paging_state, res2.rows.unwrap().len());

    let res3 = session.query_paged(paged_query.clone(), &[], res2.paging_state).await?;
    println!("Paging state: {:#?} ({} rows)", res3.paging_state, res3.rows.unwrap().len());

    let paged_prepared = session.prepare(Query::new("SELECT a, b, c FROM ks.t").with_page_size(7)).await?;

    let res4 = session.execute(&paged_prepared, &[]).await?;
    println!("Paging state from the prepared statement execution: {:#?} ({} rows)", res4.paging_state, res4.rows.unwrap().len());
    let res5 = session.execute_paged(&paged_prepared, &[], res4.paging_state).await?;
    println!("Paging state from the second prepared statement execution: {:#?} ({} rows)", res5.paging_state, res5.rows.unwrap().len());
    let res6 = session.execute_paged(&paged_prepared, &[], res5.paging_state).await?;
    println!("Paging state from the third prepared statement execution: {:#?} ({} rows)", res6.paging_state, res6.rows.unwrap().len());

    println!("Ok.");
    Ok(())
}

#[tokio::test]
async fn value_list() -> Result<(), Box<dyn std::error::Error>> {
    let session = ScyllaPool::connection().await;
    let keyspace = ScyllaPool::init_keyspace(session, "ks", 1).await?;
    let table = ScyllaPool::init_table(session, &keyspace, "my_type", "k int, my text, primary key (k)").await;

    #[derive(scylla::SerializeRow)]
    struct MyType<'a> {
        k: i32,
        my: Option<&'a str>,
    }

    let to_insert = MyType { k: 17, my: Some("Some str") };

    session.query("INSERT INTO ks.my_type (k, my) VALUES (?, ?)", to_insert).await.unwrap();

    // You can also use type generics:
    #[derive(scylla::SerializeRow)]
    struct MyTypeWithGenerics<S: scylla::serialize::value::SerializeCql> {
        k: i32,
        my: Option<S>,
    }

    let to_insert_2 = MyTypeWithGenerics { k: 18, my: Some("Some string".to_owned()) };

    session.query("INSERT INTO ks.my_type (k, my) VALUES (?, ?)", to_insert_2).await.unwrap();

    let q = session.query("SELECT * FROM ks.my_type", &[]).await.unwrap();

    println!("Q: {:?}", q.rows);

    Ok(())
}

#[tokio::test]
async fn custom_deserialization() -> Result<(), Box<dyn std::error::Error>> {
    use scylla::cql_to_rust::{FromCqlVal, FromCqlValError};
    use scylla::frame::response::result::CqlValue;
    use scylla::{impl_from_cql_value_from_method};

    let session = ScyllaPool::connection().await;
    let keyspace = ScyllaPool::init_keyspace(session, "ks", 1).await?;
    let table = ScyllaPool::init_table(session, &keyspace, "tc", "pk int PRIMARY KEY, v text").await?;

    session.query("INSERT INTO ks.tc (pk, v) VALUES (?, ?)", (1, "asdf")).await?;

    // You can implement FromCqlVal for your own types
    #[derive(PartialEq, Eq, Debug)]
    struct MyType(String);

    impl FromCqlVal<CqlValue> for MyType {
        fn from_cql(cql_val: CqlValue) -> anyhow::Result<Self, FromCqlValError> {
            Ok(Self(
                cql_val.into_string().ok_or(FromCqlValError::BadCqlType)?,
            ))
        }
    }

    let (v,) = session.query("SELECT v FROM ks.tc WHERE pk = 1", ()).await?.single_row_typed::<(MyType,)>()?;
    assert_eq!(v, MyType("asdf".to_owned()));

    // If you defined an extension trait for CqlValue then you can use
    // the `impl_from_cql_value_from_method` macro to turn it into
    // a FromCqlValue impl
    #[derive(PartialEq, Eq, Debug)]
    struct MyOtherType(String);

    trait CqlValueExt {
        fn into_my_other_type(self) -> Option<MyOtherType>;
    }

    impl CqlValueExt for CqlValue {
        fn into_my_other_type(self) -> Option<MyOtherType> {
            Some(MyOtherType(self.into_string()?))
        }
    }

    impl_from_cql_value_from_method!(MyOtherType, into_my_other_type);

    let (v,) = session.query("SELECT v FROM ks.tc WHERE pk = 1", ()).await?.single_row_typed::<(MyOtherType,)>()?;
    assert_eq!(v, MyOtherType("asdf".to_owned()));

    println!("Ok.");

    Ok(())
}

#[tokio::test]
async fn get_test() -> Result<(), Box<dyn std::error::Error>> {
    use anyhow::anyhow;

    tracing_subscriber::fmt::init();
    let session = ScyllaPool::connection().await;

    ScyllaPool::init_keyspace(session, "ks", 1).await?;
    ScyllaPool::init_table(session, "ks", "hello", "pk int, ck int, value text, primary key (pk, ck)").await?;

    session.query("INSERT INTO ks.hello (pk, ck, value) VALUES (?, ?, ?)", (3, 4, "def")).await?;
    session.query("INSERT INTO ks.hello (pk, ck, value) VALUES (1, 2, 'abc')", &[]).await?;

    let query_result = session.query("SELECT pk, ck, value FROM ks.hello", &[]).await?;
    let (ck_idx, _) = query_result.get_column_spec("ck").ok_or_else(|| anyhow!("No ck column found"))?;
    let (value_idx, _) = query_result.get_column_spec("value").ok_or_else(|| anyhow!("No value column found"))?;
    println!("ck           |  value");
    println!("---------------------");
    for row in query_result.rows.ok_or_else(|| anyhow!("no rows found"))? {
        println!("{:?} | {:?}", row.columns[ck_idx], row.columns[value_idx]);
    }

    Ok(())
}

#[tokio::test]
async fn cql_time_type() -> Result<(), Box<dyn std::error::Error>> {
    use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
    use scylla::frame::response::result::CqlValue;
    use scylla::frame::value::{CqlDate, CqlTime, CqlTimestamp};
    use scylla::transport::session::{IntoTypedRows};

    let session = ScyllaPool::connection().await;
    ScyllaPool::init_keyspace(session, "ks", 1).await?;

    // Date
    // Date is a year, month and day in the range -5877641-06-23 to -5877641-06-23
    ScyllaPool::init_table(session, "ks", "dates", "d date primary key").await?;
    // If 'chrono' feature is enabled, dates in the range -262145-1-1 to 262143-12-31 can be represented using
    // chrono::NaiveDate
    let chrono_date = NaiveDate::from_ymd_opt(2020, 2, 20).unwrap();
    session.query("INSERT INTO ks.dates (d) VALUES (?)", (chrono_date, )).await?;
    if let Some(rows) = session.query("SELECT d from ks.dates", &[]).await?.rows {
        for row in rows.into_typed::<(NaiveDate, )>() {
            let (read_date, ): (NaiveDate, ) = match row {
                Ok(read_date) => read_date,
                Err(_) => continue, // We might read a date that does not fit in NaiveDate, skip it
            };

            println!("Parsed a date into chrono::NaiveDate: {:?}", read_date);
        }
    }

    // Alternatively, you can enable 'time' feature and use `time::Date` to represent date. `time::Date` only allows dates in range -9999-1-1 to 9999-12-31.
    // Or, if you have 'time/large-dates' feature enabled, this range changes to -999999-1-1 to 999999-12-31
    let time_date = time::Date::from_calendar_date(2020, time::Month::March, 21).unwrap();
    session.query("INSERT INTO ks.dates (d) VALUES (?)", (time_date, )).await?;
    if let Some(rows) = session.query("SELECT d from ks.dates", &[]).await?.rows {
        for row in rows.into_typed::<(time::Date, )>() {
            let (read_date, ) = match row {
                Ok(read_date) => read_date,
                Err(_) => continue, // We might read a date that does not fit in time::Date, skip it
            };

            println!("Parsed a date into time::Date: {:?}", read_date);
        }
    }

    // Dates outside this range must be represented in the raw form - an u32 describing days since -5877641-06-23
    let example_big_date: CqlDate = CqlDate(u32::MAX);
    session.query("INSERT INTO ks.dates (d) VALUES (?)", (example_big_date, )).await?;
    if let Some(rows) = session.query("SELECT d from ks.dates", &[]).await?.rows {
        for row in rows {
            let read_days: u32 = match row.columns[0] {
                Some(CqlValue::Date(CqlDate(days))) => days,
                _ => panic!("oh no"),
            };

            println!("Read a date as raw days: {}", read_days);
        }
    }

    // Time
    // Time is represented as nanosecond count since midnight in range 0..=86399999999999
    ScyllaPool::init_table(session, "ks", "times", "t time primary key").await?;

    // Time can be represented using 3 different types, chrono::NaiveTime, time::Time and CqlTime. All types support full value range

    // chrono::NaiveTime
    let chrono_time = NaiveTime::from_hms_nano_opt(1, 2, 3, 456_789_012).unwrap();
    session.query("INSERT INTO ks.times (t) VALUES (?)", (chrono_time, )).await?;
    if let Some(rows) = session.query("SELECT t from ks.times", &[]).await?.rows {
        for row in rows.into_typed::<(NaiveTime, )>() {
            let (read_time, ) = row?;

            println!("Parsed a time into chrono::NaiveTime: {:?}", read_time);
        }
    }

    // time::Time
    let time_time = time::Time::from_hms_nano(2, 3, 4, 567_890_123).unwrap();
    session.query("INSERT INTO ks.times (t) VALUES (?)", (time_time, )).await?;
    if let Some(rows) = session.query("SELECT t from ks.times", &[]).await?.rows {
        for row in rows.into_typed::<(time::Time, )>() {
            let (read_time, ) = row?;

            println!("Parsed a time into time::Time: {:?}", read_time);
        }
    }

    // CqlTime
    let time_time = CqlTime(((3 * 60 + 4) * 60 + 5) * 1_000_000_000 + 678_901_234);
    session.query("INSERT INTO ks.times (t) VALUES (?)", (time_time, )).await?;
    if let Some(rows) = session.query("SELECT t from ks.times", &[]).await?.rows {
        for row in rows.into_typed::<(CqlTime, )>() {
            let (read_time, ) = row?;

            println!("Read a time as raw nanos: {:?}", read_time);
        }
    }

    // Timestamp
    // Timestamp is represented as milliseconds since unix epoch - 1970-01-01. Negative values are also possible
    ScyllaPool::init_table(session, "ks", "timestamps", "t timestamp primary key").await?;

    // Timestamp can also be represented using 3 different types,
    // chrono::DateTime<chrono::Utc>, time::OffsetDateTime and CqlTimestamp.
    // Only CqlTimestamp allows full range.

    // chrono::DateTime<chrono::Utc>
    let chrono_datetime = Utc::now();
    session.query("INSERT INTO ks.timestamps (t) VALUES (?)", (chrono_datetime, )).await?;
    if let Some(rows) = session.query("SELECT t from ks.timestamps", &[]).await?.rows {
        for row in rows.into_typed::<(DateTime<Utc>, )>() {
            let (read_time, ) = row?;

            println!("Parsed a timestamp into chrono::DateTime<chrono::Utc>: {:?}", read_time);
        }
    }

    // time::OffsetDateTime
    let time_datetime = time::OffsetDateTime::now_utc();
    session.query("INSERT INTO ks.timestamps (t) VALUES (?)", (time_datetime, )).await?;
    if let Some(rows) = session.query("SELECT t from ks.timestamps", &[]).await?.rows {
        for row in rows.into_typed::<(time::OffsetDateTime, )>() {
            let (read_time, ) = row?;

            println!("Parsed a timestamp into time::OffsetDateTime: {:?}", read_time);
        }
    }

    // CqlTimestamp
    let cql_datetime = CqlTimestamp(1 << 31);
    session.query("INSERT INTO ks.timestamps (t) VALUES (?)", (cql_datetime, )).await?;
    if let Some(rows) = session.query("SELECT t from ks.timestamps", &[]).await?.rows {
        for row in rows.into_typed::<(CqlTimestamp, )>() {
            let (read_time, ) = row?;

            println!("Read a timestamp as raw millis: {:?}", read_time);
        }
    }

    Ok(())
}
