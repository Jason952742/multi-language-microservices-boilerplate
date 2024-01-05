use uuid::Uuid;

// External services must be called during the processing of the command.
pub struct AtmError;

pub struct CheckingError;

// A very simple "happy path" set of services that always succeed.
pub struct HappyPathAccountServices;

impl HappyPathAccountServices {
    pub async fn atm_withdrawal_atm_id(_atm_id: &str, _amount: f64) -> Result<(), AtmError> {
        Ok(())
    }

    pub async fn validate_check(_account_id: &Uuid, _check_number: &str) -> Result<(), CheckingError> {
        Ok(())
    }
}
