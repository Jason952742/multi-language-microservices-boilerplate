use crate::domain::commands::account_cmd::AccountCommand;
use crate::domain::services::AccountService;
use tokio::sync::mpsc;

pub struct AccountActor {
  receiver: mpsc::Receiver<AccountCommand>,
}

impl AccountActor {
  pub(crate) fn new(receiver: mpsc::Receiver<AccountCommand>) -> Self {
    AccountActor { receiver }
  }

  async fn handle_message(&mut self, command: AccountCommand) {
    match command {
      AccountCommand::Create { id, user_id, ccy_type, resp } => {
        let res = AccountService::create_account(id, user_id, ccy_type).await;
        let _ = resp.send(res);
      }
      AccountCommand::Update { id, account_type, account_name, description, resp } => {
        let res = AccountService::update_account(id, account_type, account_name, description).await;
        let _ = resp.send(res);
      }
      AccountCommand::Disable { id, resp } => {
        let res = AccountService::disabled_account(id).await;
        let _ = resp.send(res);
      }
    }
  }
}

pub async fn run_account_actor(mut actor: AccountActor) {
  while let Some(msg) = actor.receiver.recv().await {
    actor.handle_message(msg).await;
  }
}
