use shared::consul_api;

pub mod repositories;
pub mod dto;
pub mod cache;

pub async fn discovery(service_name: &str) -> Result<String, String> {
    let opt = consul_api::ConsulOption::default();
    let cs = consul_api::Consul::new(opt).unwrap();
    let filter = consul_api::Filter::Service(service_name.into());
    let srv = cs
        .get_service(&filter)
        .await
        .map_err(|err| err.to_string())?;
    if let Some(srv) = srv {
        return Ok(format!("http://{}:{}", srv.address, srv.port));
    }
    Err("no service".to_string())
}
