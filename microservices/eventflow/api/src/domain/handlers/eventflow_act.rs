use tokio::sync::mpsc;
use uuid::Uuid;
use crate::domain::commands::eventflow_cmd::EventflowCommand;

pub struct EventflowActor {
    receiver: mpsc::Receiver<EventflowCommand>,
}

impl EventflowActor {
    pub(crate) fn new(receiver: mpsc::Receiver<EventflowCommand>) -> Self {
        EventflowActor { receiver }
    }

    async fn handle_message(&mut self, command: EventflowCommand) {
        match command {
            EventflowCommand::CreateUser { user_id, user_name, resp } => {
                // let res = MemberService::create_referral(user_id, event).await;
                // let _ = resp.send(res);
                todo!()
            }
            EventflowCommand::AccountDeposit { user_id, account_id, payment, resp } => {
                // let res = MemberService::update_referral(user_id, member_type, level, active, description).await;
                // let _ = resp.send(res);
                todo!()
            }
            EventflowCommand::AccountWithdraw { user_id, account_id, payment, resp } => {
                // let res = MemberService::update_referral(user_id, member_type, level, active, description).await;
                // let _ = resp.send(res);
                todo!()
            }
            EventflowCommand::MemberSubscribe { user_id, member_id, payments, duration, resp } => {
                // let res = MemberService::bind_referral(user_id, referral_id).await;
                // let _ = resp.send(res);
                todo!()
            }
        }
    }
}

pub async fn run_eventflow_actor(mut actor: EventflowActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}
