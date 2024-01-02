use tokio::sync::mpsc;
use crate::domain::commands::member_cmd::MemberCommand;
use crate::domain::services::MemberService;

pub struct MemberActor {
    receiver: mpsc::Receiver<MemberCommand>,
}

impl MemberActor {
    pub(crate) fn new(receiver: mpsc::Receiver<MemberCommand>) -> Self {
        MemberActor { receiver }
    }

    async fn handle_message(&mut self, command: MemberCommand) {
        match command {
            MemberCommand::Create { user_id, event, resp } => {
                let res = MemberService::create_referral(user_id, event).await;
                let _ = resp.send(res);
            }
            MemberCommand::Update { user_id, member_type, level, active, description, resp } => {
                let res = MemberService::update_referral(user_id, member_type, level, active, description).await;
                let _ = resp.send(res);
            }
            MemberCommand::Bind { user_id, referral_id, resp } => {
                let res = MemberService::bind_referral(user_id, referral_id).await;
                let _ = resp.send(res);
            }
        }
    }
}

pub async fn run_member_actor(mut actor: MemberActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}
