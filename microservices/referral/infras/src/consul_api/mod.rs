mod model;

use std::time::Duration;
use tokio::time::{Instant, interval};

pub use model::*;

pub struct Consul {
    option: ConsulOption,
    client: reqwest::Client,
}

impl Consul {
    pub fn new(option: ConsulOption) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(option.timeout_sec))
            .build()?;
        Ok(Self { option, client })
    }

    fn api_url(&self, api_name: &str) -> String {
        format!("{}://{}/v1/agent/{}", &self.option.protocol, &self.option.addr, api_name)
    }

    pub async fn register(&self, registration: &Registration) -> Result<(), reqwest::Error> {
        self.client
            .put(self.api_url("service/register"))
            .json(registration)
            .send()
            .await?;
        Ok(())
    }

    pub async fn deregister(&self, service_id: &str) -> Result<(), reqwest::Error> {
        let deregister_api = format!("service/deregister/{}", service_id);
        self.client
            .put(self.api_url(&deregister_api))
            .json(&())
            .send()
            .await?;
        Ok(())
    }

    pub async fn services(&self) -> Result<Services, reqwest::Error> {
        let list: Services = self
            .client
            .get(self.api_url("services"))
            .send()
            .await?
            .json()
            .await?;
        Ok(list)
    }

    pub async fn get_service(&self, filter: &Filter) -> Result<Option<Service>, reqwest::Error> {
        let list = self.services().await?;
        for (_, s) in list {
            let has = match &filter {
                &Filter::ID(id) => id == &s.id,
                &Filter::Service(srv) => srv == &s.service,
            };
            if has {
                return Ok(Some(s));
            }
        }
        Ok(None)
    }

    pub async fn discover_service(&self) -> Result<i32, reqwest::Error> {
        let mut interval = interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            // Execute discover task when the timer is triggered
             let filter = Filter::Service(ServiceName::MuReferral.to_string());
            let memberService = self.get_service(&filter).await;
            println!("{:?}", memberService.unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_services() {
        let opt = ConsulOption::default();
        let cs = Consul::new(opt);
        assert!(cs.is_ok());
        let cs = cs.unwrap();
        let all_services = cs.services().await;
        assert!(all_services.is_ok());
        let all_services = all_services.unwrap();
        for (_, srv) in &all_services {
            println!("{:?}", srv);
        }
    }

    #[tokio::test]
    async fn test_register_service() {
        let opt = ConsulOption::default();
        let cs = Consul::new(opt);
        assert!(cs.is_ok());
        let cs = cs.unwrap();
        let registration = Registration::simple_with_tags(
            ServiceName::MuMember,
            vec!["axum", "tokio", "grpc", "tonic"],
            "127.0.0.1",
            12345,
        );
        let r = cs.register(&registration).await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_deregister_service() {
        let opt = ConsulOption::default();
        let cs = Consul::new(opt);
        assert!(cs.is_ok());
        let cs = cs.unwrap();

        let r = cs.deregister("axum.rs").await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_get_services() {
        let opt = ConsulOption::default();
        let cs = Consul::new(opt);
        assert!(cs.is_ok());
        let cs = cs.unwrap();
        let filter = Filter::ID("axum.rs".to_string());
        let srv = cs.get_service(&filter).await;
        assert!(srv.is_ok());
        let srv = srv.unwrap();
        assert!(srv.is_some());
        let srv = srv.unwrap();
        println!("{:?}", srv);
    }
}
