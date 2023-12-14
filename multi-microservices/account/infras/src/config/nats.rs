use async_nats::jetstream::consumer::{AckPolicy, PullConsumer};
use async_nats::jetstream::Context;
use async_nats::{jetstream, Client};
use std::env;
use strum_macros;
use tokio::sync::OnceCell;

#[derive(Debug, strum_macros::Display)]
pub enum NatsMessage {
    AccountInit,
    ReferralInit
}

#[derive(Debug)]
pub struct Nats;

static CLIENT: OnceCell<Client> = OnceCell::const_new();
static JETSTREAM: OnceCell<Context> = OnceCell::const_new();

static ACCOUNT_INIT: OnceCell<PullConsumer> = OnceCell::const_new();
static REFERRAL_INIT: OnceCell<PullConsumer> = OnceCell::const_new();

impl Nats {
    pub async fn client() -> &'static Client {
        CLIENT
            .get_or_init(|| async {
                dotenvy::dotenv().ok();
                let host = env::var("NATS_HOST").expect("NATS_HOST must be set");
                let nats_url = format!("nats://{host}:4222");
                let nats_client = async_nats::connect(nats_url).await;
                nats_client.expect("nats connect failed")
            })
            .await
    }

    pub async fn jetstream() -> &'static Context {
        JETSTREAM
            .get_or_init(|| async {
                let client = Self::client().await;
                let jetstream = jetstream::new(client.clone());
                jetstream
            })
            .await
    }

    pub async fn init_stream() -> Result<(), async_nats::Error> {
        let jetstream = Self::jetstream().await;

        jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: NatsMessage::AccountInit.to_string(),
                subjects: vec![format!("{}.>", NatsMessage::AccountInit)],
                ..Default::default()
            })
            .await
            .expect("Failed to create account initialisation stream");

        jetstream
            .get_or_create_stream(jetstream::stream::Config {
                name: NatsMessage::ReferralInit.to_string(),
                subjects: vec![format!("{}.>", NatsMessage::ReferralInit)],
                ..Default::default()
            })
            .await
            .expect("Failed to create recommended initialisation stream");

        Ok(())
    }

    pub async fn account_init_consumer() -> &'static PullConsumer {
        ACCOUNT_INIT.get_or_init(|| Self::get_consumer(NatsMessage::AccountInit)).await
    }

    pub async fn referral_init_consumer() -> &'static PullConsumer {
        REFERRAL_INIT.get_or_init(|| Self::get_consumer(NatsMessage::ReferralInit)).await
    }

    /// Getting or Creating Consumers
    async fn get_consumer(stream: NatsMessage) -> PullConsumer {
        let jetstream = Self::jetstream().await;
        let stream = jetstream.get_stream(stream.to_string()).await.expect("Failed to initialise stream");
        stream
            .create_consumer(jetstream::consumer::pull::Config {
                // durable_name: Some(format!("consumer_{:?}", stream)),
                durable_name: Some("consumer".to_string()),

                // ack_policy - AckExplicit|AckNone|AckAll
                ack_policy: AckPolicy::Explicit,
                max_ack_pending: 2000,
                ..Default::default()
            })
            .await
            .unwrap()
    }
}
