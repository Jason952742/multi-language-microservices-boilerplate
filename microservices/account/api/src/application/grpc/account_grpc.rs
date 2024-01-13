use crate::application::grpc::account_grpc::account_proto::{
  account_server, AccountId, AccountInfo, AccountListReply, AccountReply, AddAccountRequest, ListRequest, ProcessStatusReply, UpdateAccountRequest, UserId,
};
use crate::domain::commands::account_cmd::{AccountCommand, AccountEvent};
use crate::domain::entities::account;
use crate::domain::entities::enums::{AccountStatus, AccountType, CurrencyType};
use crate::domain::handlers::{run_account_actor, AccountActor};
use crate::domain::queries::account_qry::AccountQuery;
use rust_decimal::prelude::ToPrimitive;
use sea_orm::ActiveEnum;
use shared::utils::{parse_code, to_uuid};
use std::str::FromStr;
use tokio::sync::{mpsc, oneshot};
use tonic::{Code, Request, Response, Status};

pub mod account_proto {
  tonic::include_proto!("account");
}

#[derive(Debug)]
pub struct AccountGrpc {
  tx: mpsc::Sender<AccountCommand>,
}

impl AccountGrpc {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel(32);
    let actor = AccountActor::new(rx);
    tokio::spawn(run_account_actor(actor));
    Self { tx }
  }
}

#[tonic::async_trait]
impl account_server::Account for AccountGrpc {
  // #[tracing::instrument]
  async fn add_account(&self, request: Request<AddAccountRequest>) -> Result<Response<ProcessStatusReply>, Status> {
    let request = request.into_inner();
    tracing::info!("add account: {:?}", &request);

    let (resp_tx, resp_rx) = oneshot::channel();
    let command =
      AccountCommand::Create { id: to_uuid(&request.id), user_id: to_uuid(&request.user_id), ccy_type: CurrencyType::from_str(&request.ccy_type).unwrap(), resp: resp_tx };
    if self.tx.send(command).await.is_err() {
      eprintln!("connection task shutdown");
    }
    match resp_rx.await.unwrap() {
      Ok(event) => match event {
        AccountEvent::Created { id } => Ok(to_process_reply(id.to_string())),
        _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
      },
      Err(e) => Err(e),
    }
  }

  // #[tracing::instrument]
  async fn disable_account(&self, request: Request<AccountId>) -> Result<Response<ProcessStatusReply>, Status> {
    let request = request.into_inner();
    tracing::info!("disable account: {:?}", &request);

    let (resp_tx, resp_rx) = oneshot::channel();
    let command = AccountCommand::Disable { id: to_uuid(&request.id), resp: resp_tx };
    if self.tx.send(command).await.is_err() {
      eprintln!("connection task shutdown");
    }
    match resp_rx.await.unwrap() {
      Ok(event) => match event {
        AccountEvent::Disabled => Ok(to_process_reply(request.id)),
        _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
      },
      Err(e) => Err(e),
    }
  }

  // #[tracing::instrument]
  async fn update_account(&self, request: Request<UpdateAccountRequest>) -> Result<Response<ProcessStatusReply>, Status> {
    let request = request.into_inner();
    tracing::info!("update account: {:?}", &request);

    let (resp_tx, resp_rx) = oneshot::channel();
    let command = AccountCommand::Update {
      id: to_uuid(&request.id),
      account_type: AccountType::from_str(&request.account_type).unwrap(),
      account_name: request.account_name,
      description: request.description,
      resp: resp_tx,
    };
    if self.tx.send(command).await.is_err() {
      eprintln!("connection task shutdown");
    }
    match resp_rx.await.unwrap() {
      Ok(event) => match event {
        AccountEvent::Updated => Ok(to_process_reply(request.id)),
        _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
      },
      Err(e) => Err(e),
    }
  }

  // #[tracing::instrument]
  async fn get_accounts(&self, request: Request<ListRequest>) -> Result<Response<AccountListReply>, Status> {
    let request = request.into_inner();
    tracing::info!("get accounts: {:?}", &request);

    let res = AccountQuery::get_accounts(
      request.page,
      request.per_page,
      request.status.map(|x| AccountStatus::from_str(&x).unwrap()),
      request.ccy_type.map(|x| CurrencyType::from_str(&x).unwrap()),
      request.account_type.map(|x| AccountType::from_str(&x).unwrap()),
    )
    .await?;
    Ok(to_account_list_reply(res.0, res.1))
  }

  // #[tracing::instrument]
  async fn get_accounts_by_user_id(&self, request: Request<UserId>) -> Result<Response<AccountListReply>, Status> {
    let request = request.into_inner();
    tracing::info!("get accounts by user_id: {:?}", &request);

    let res = AccountQuery::get_accounts_by_user_id(to_uuid(&request.id)).await?;
    Ok(to_account_list_reply(res, 1))
  }

  async fn get_account_by_id(&self, request: Request<AccountId>) -> Result<Response<AccountReply>, Status> {
    let request = request.into_inner();
    tracing::info!("get account by user_id: {:?}", &request);

    match AccountQuery::get_account_by_id(to_uuid(&request.id)).await? {
      None => Err(Status::not_found(request.id)),
      Some(m) => Ok(to_account_reply(m)),
    }
  }
}

fn to_process_reply(process_id: String) -> Response<ProcessStatusReply> {
  Response::new(ProcessStatusReply { code: parse_code(Code::Ok), message: "Processed".to_string(), success: true, process_id })
}

fn to_account_reply(model: account::Model) -> Response<AccountReply> {
  let r = AccountReply { code: parse_code(Code::Ok), message: "account".to_string(), data: Some(model.into()) };
  Response::new(r)
}

fn to_account_list_reply(models: Vec<account::Model>, num_pages: u64) -> Response<AccountListReply> {
  let list = models.clone().into_iter().map(|x| x.into()).collect();
  let r = AccountListReply { code: parse_code(Code::Ok), message: "account list".to_string(), data: list, num_pages };
  Response::new(r)
}

impl Into<AccountInfo> for account::Model {
  fn into(self) -> AccountInfo {
    AccountInfo {
      user_id: self.user_id.to_string(),
      account_id: self.id.to_string(),
      account_name: self.account_name,
      status: self.status.to_value(),
      account_type: self.account_type.to_value(),
      ccy_type: self.ccy_type.to_value(),
      frozen_amount: self.frozen_amount.to_f64().unwrap(),
      balance: self.balance.to_f64().unwrap(),
    }
  }
}
