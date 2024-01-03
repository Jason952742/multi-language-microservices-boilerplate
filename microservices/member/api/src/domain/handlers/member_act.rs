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
            MemberCommand::Create { user_id, user_name, resp } => {
                let res = MemberService::create_member(user_id, user_name).await;
                let _ = resp.send(res);
            }
            MemberCommand::Update { user_id, member_type, level, active, description, resp } => {
                let res = MemberService::update_member(user_id, member_type, level, active, description).await;
                let _ = resp.send(res);
            }
            MemberCommand::Disable { user_id, resp } => {
                let res = MemberService::disabled_member(user_id).await;
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
