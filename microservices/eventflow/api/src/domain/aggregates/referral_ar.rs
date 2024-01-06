use std::fmt::{Display, Formatter};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Referral {
    pub user_id: Uuid,
    pub referral_code: String,
    pub referrals: Vec<Uuid>,
    pub referrer_id: Option<Uuid>,
    pub referrer_code: Option<String>,
}

impl Referral {
    pub const TABLE_NAME: &'static str = "referral_events";

    pub fn new(id: &Uuid) -> Referral {
        Self { user_id: *id, ..Default::default() }
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    pub async fn handle(&self, command: ReferralCommand) -> Result<ReferralEvent, ReferralError> {
        match command {
            ReferralCommand::CreateReferral { user_id, referral_code, referrer_id, referrer_code } => {
                Ok(ReferralEvent::ReferralCreated { user_id, referral_code, referrer_id, referrer_code })
            }
            ReferralCommand::UserRegistered { referred_id } => {
                Ok(ReferralEvent::ReferralBound { referred_id })
            }
        }
    }

    /// Reconstructing the domain model
    pub fn apply(&mut self, event: ReferralEvent) {
        match event {
            ReferralEvent::ReferralCreated { user_id, referral_code, referrer_id, referrer_code } => {
                self.user_id = user_id;
                self.referral_code = referral_code;
                self.referrer_id = referrer_id;
                self.referrer_code = referrer_code;
            }
            ReferralEvent::ReferralBound { referred_id } => {
                self.referrals.push(referred_id);
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ReferralCommand {
    CreateReferral { user_id: Uuid, referral_code: String, referrer_id: Option<Uuid>, referrer_code: Option<String> },
    UserRegistered { referred_id: Uuid, },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReferralEvent {
    ReferralCreated { user_id: Uuid, referral_code: String, referrer_id: Option<Uuid>, referrer_code: Option<String> },
    ReferralBound { referred_id: Uuid },
}

impl ReferralEvent {
    pub fn event_type(&self) -> String {
        match self {
            ReferralEvent::ReferralCreated { .. } => "ReferralCreated".to_string(),
            ReferralEvent::ReferralBound { .. } => "ReferralBound".to_string(),
        }
    }

    pub fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

impl Into<serde_json::Value> for ReferralEvent {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(&json!(self)).expect("Error decoding payload")
    }
}

impl From<serde_json::Value> for ReferralEvent {
    fn from(v: serde_json::Value) -> Self {
        serde_json::from_value::<ReferralEvent>(v).unwrap()
    }
}

#[derive(Debug)]
pub struct ReferralError(String);

impl From<&str> for ReferralError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for ReferralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ReferralError {}
