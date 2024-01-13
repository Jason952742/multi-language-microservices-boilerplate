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
            MemberCommand::Create { id, user_id, user_name, sub_end_date, resp } => {
                let res = MemberService::create_member(id, user_id, user_name, sub_end_date).await;
                let _ = resp.send(res);
            }
            MemberCommand::Subscribe { id, user_id, sub_end_date, resp } => {
                let res = MemberService::subscribe(id, user_id, sub_end_date).await;
                let _ = resp.send(res);
            }
            MemberCommand::Update { id, member_type, level, active, description, resp } => {
                let res = MemberService::update_member(id, member_type, level, active, description).await;
                let _ = resp.send(res);
            }
            MemberCommand::Disable { id, resp } => {
                let res = MemberService::disabled_member(id).await;
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
