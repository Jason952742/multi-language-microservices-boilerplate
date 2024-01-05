use tokio::sync::mpsc;
use crate::domain::commands::eventflow_cmd::EventflowCommand;
use crate::domain::services::TransactionService;

pub struct EventflowActor {
    receiver: mpsc::Receiver<EventflowCommand>,
}

impl EventflowActor {
    pub(crate) fn new(receiver: mpsc::Receiver<EventflowCommand>) -> Self {
        EventflowActor { receiver }
    }

    async fn handle_message(&mut self, command: EventflowCommand) {
        match command {
            EventflowCommand::CreateUser { user_id, user_name, data, resp } => {
                let res = TransactionService::create_user(user_id, user_name, data).await;
                let _ = resp.send(res);
            }
            EventflowCommand::AccountDeposit { account_id, payment, resp } => {
                let res = TransactionService::account_deposit(account_id, payment).await;
                let _ = resp.send(res);
            }
            EventflowCommand::AccountWithdraw { account_id, payment, resp } => {
                let res = TransactionService::account_withdraw(account_id, payment).await;
                let _ = resp.send(res);
            }
            EventflowCommand::MemberSubscribe { member_id, payments, duration, resp } => {
                let res = TransactionService::member_subscribe(member_id, payments, duration).await;
                let _ = resp.send(res);
            }
        }
    }
}

pub async fn run_eventflow_actor(mut actor: EventflowActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}
