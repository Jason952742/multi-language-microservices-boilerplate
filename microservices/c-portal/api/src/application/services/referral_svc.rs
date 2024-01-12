use uuid::Uuid;
use shared::utils::{CustomError, to_uuid};
use crate::application::grpc::referral_client;
use crate::infra::cache::referral_cache;

pub async fn get_referral(code: &str) -> Result<Option<Uuid>, CustomError> {
    let user_id = referral_cache::get_referral(code).await?;
    if let Some(id) = user_id {
        Ok(Some(id))
    } else {
        let result = referral_client::get_referral_by_code(code.to_string()).await?;
        match result.data {
            None => Ok(None),
            Some(user) => {
                let id = to_uuid(&user.user_id);
                referral_cache::set_referral(code, id.clone()).await?;
                Ok(Some(id))
            }
        }
    }
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = get_referral("N43kHRwcSjSLmUA7duGucA").await?;

    print!("{:?}", result);

    Ok(())
}