use tokio::sync::mpsc;
use crate::domain::commands::member_cmd::ReferralCommand;
use crate::domain::services::MemberService;

pub struct ReferralActor {
    receiver: mpsc::Receiver<ReferralCommand>,
}

impl ReferralActor {
    pub(crate) fn new(receiver: mpsc::Receiver<ReferralCommand>) -> Self {
        ReferralActor { receiver }
    }

    async fn handle_message(&mut self, command: ReferralCommand) {
        match command {
            ReferralCommand::Create { user_id, event, resp } => {
                let res = MemberService::create_referral(user_id, event).await;
                let _ = resp.send(res);
            }
            ReferralCommand::Update { user_id, member_type, level, active, description, resp } => {
                let res = MemberService::update_referral(user_id, member_type, level, active, description).await;
                let _ = resp.send(res);
            }
        }
    }
}

pub async fn run_referral_actor(mut actor: ReferralActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}
