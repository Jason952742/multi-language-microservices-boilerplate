use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use tokio::sync::{mpsc, oneshot};
use tonic::{Code, Request, Response, Status};
use shared::{parse_code, to_datetime, to_uuid};
use crate::application::grpc::eventflow_grpc::eventflow_proto::{AccountTransferRequest, AccountTransactionReply, eventflow_server, ListRequest, MemberSubscriptionReply, MemberSubscriptionRequest, TransactionId, TransactionInfo, TransactionListReply, TransactionReply, UserCreatedReply, UserCreateRequest, UserInfo};
use crate::domain::commands::eventflow_cmd::{EventflowCommand, EventflowEvent};
use crate::domain::entities::enums::{CurrencyType, PaymentType, TransactionType};
use crate::domain::entities::transaction;
use crate::domain::entities::valobj::{Payment, User};
use crate::domain::handlers::{EventflowActor, run_eventflow_actor};
use crate::domain::queries::transaction_qry::TransactionQuery;

pub mod eventflow_proto {
    tonic::include_proto!("eventflow");
}

#[derive(Debug)]
pub struct EventflowGrpc {
    tx: mpsc::Sender<EventflowCommand>,
}

impl EventflowGrpc {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        let actor = EventflowActor::new(rx);
        tokio::spawn(run_eventflow_actor(actor));
        Self { tx }
    }
}

#[tonic::async_trait]
impl eventflow_server::Eventflow for EventflowGrpc {

    // #[tracing::instrument]
    async fn get_transaction_by_id(&self, request: Request<TransactionId>) -> Result<Response<TransactionReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get transaction by id: {:?}", &request);

        match TransactionQuery::get_transaction_by_id(to_uuid(&request.id)).await? {
            None => Err(Status::not_found(request.id)),
            Some(m) => Ok(to_transaction_reply(m))
        }
    }

    // #[tracing::instrument]
    async fn get_transactions(&self, request: Request<ListRequest>) -> Result<Response<TransactionListReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get transactions: {:?}", &request);

       let res = TransactionQuery::get_transactions(
            to_uuid(&request.user_id),
            TransactionType::from_str(&request.transaction_type).unwrap()).await?;

       Ok(to_transaction_list_reply(res))
    }

    // #[tracing::instrument]
    async fn user_create(&self, request: Request<UserCreateRequest>) -> Result<Response<UserCreatedReply>, Status> {
        let request = request.into_inner();
        tracing::info!("user create: {:?}", &request);
        let data = format!("{:?}", &request);

        let (resp_tx, resp_rx) = oneshot::channel();
        let command = EventflowCommand::CreateUser {
            user_id: to_uuid(&request.user_id),
            user_name: request.user_name,
            data,
            resp: resp_tx
        };
        if self.tx.send(command).await.is_err() {
            eprintln!("connection task shutdown");
        }
        match resp_rx.await.unwrap() {
            Ok(event) => match event {
                EventflowEvent::Created { user } => Ok(to_user_reply(user)),
                _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
            },
            Err(e) => Err(e),
        }
    }

    // #[tracing::instrument]
    async fn account_deposit(&self, request: Request<AccountTransferRequest>) -> Result<Response<AccountTransactionReply>, Status> {
        let request = request.into_inner();
        tracing::info!("account deposit: {:?}", &request);

        let (resp_tx, resp_rx) = oneshot::channel();
        let command = EventflowCommand::AccountDeposit {
            account_id: to_uuid(&request.account_id),
            payment: Payment {
                payment_type: PaymentType::from_str(&request.payment.payment_type).unwrap(),
                currency_type: CurrencyType::from_str(&request.payment.currency_type).unwrap(),
                amount: Decimal::from_f64(request.payment.amount).unwrap(),
                paid_at: to_datetime(&request.payment.paid_at),
                receipt: request.payment.receipt,
                equipment_id: request.payment.equipment_id,
            },
            resp: resp_tx
        };
        if self.tx.send(command).await.is_err() {
            eprintln!("connection task shutdown");
        }
        match resp_rx.await.unwrap() {
            Ok(event) => match event {
                EventflowEvent::AccountDeposited { balance, .. } => Ok(to_account_reply(balance)),
                _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
            },
            Err(e) => Err(e),
        }
    }

    // #[tracing::instrument]
    async fn account_withdraw(&self, request: Request<AccountTransferRequest>) -> Result<Response<AccountTransactionReply>, Status> {
        let request = request.into_inner();
        tracing::info!("account withdraw: {:?}", &request);

        let (resp_tx, resp_rx) = oneshot::channel();
        let command = EventflowCommand::AccountWithdraw {
            account_id: to_uuid(&request.account_id),
            payment: Payment {
                payment_type: PaymentType::from_str(&request.payment.payment_type).unwrap(),
                currency_type: CurrencyType::from_str(&request.payment.currency_type).unwrap(),
                amount: Decimal::from_f64(request.payment.amount).unwrap(),
                paid_at: to_datetime(&request.payment.paid_at),
                receipt: request.payment.receipt,
                equipment_id: request.payment.equipment_id,
            },
            resp: resp_tx
        };
        if self.tx.send(command).await.is_err() {
            eprintln!("connection task shutdown");
        }
        match resp_rx.await.unwrap() {
            Ok(event) => match event {
                EventflowEvent::AccountWithdrew { balance, .. } => Ok(to_account_reply(balance)),
                _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
            },
            Err(e) => Err(e),
        }
    }

    // #[tracing::instrument]
    async fn member_subscription(&self, request: Request<MemberSubscriptionRequest>) -> Result<Response<MemberSubscriptionReply>, Status> {
        let request = request.into_inner();
        tracing::info!("member subscription: {:?}", &request);

        todo!()
    }
}

fn to_account_reply(balance: Decimal) -> Response<AccountTransactionReply> {
    let res = AccountTransactionReply { code: parse_code(Code::Ok), message: "account".to_string(), success: true, balance: balance.to_f64().unwrap() };
    Response::new(res)
}

fn to_transaction_reply(model: transaction::Model) -> Response<TransactionReply> {
    let res = TransactionReply { code: parse_code(Code::Ok), message: "member".to_string(), data: Some(model.into()) };
    Response::new(res)
}

fn to_transaction_list_reply(models: Vec<transaction::Model>) -> Response<TransactionListReply> {
    let list = models.clone().into_iter().map(|x| x.into()).collect();
    let res = TransactionListReply { code: parse_code(Code::Ok), message: "member list".to_string(), data: list };
    Response::new(res)
}

impl Into<TransactionInfo> for transaction::Model {
    fn into(self) -> TransactionInfo {
        TransactionInfo {
            id: self.id.to_string(),
            user_id: self.user_id.to_string(),
            status: self.status.to_string(),
            transaction_type: self.transaction_type.to_string(),
            value: self.payload,
            rollback_id: self.rollback_id.map(|x| x.to_string()),
            description: self.description,
            created_at: self.created_at.to_string(),
        }
    }
}


fn to_user_reply(model: User) -> Response<UserCreatedReply> {
    let res = UserCreatedReply { code: parse_code(Code::Ok), message: "member".to_string(), data: model.into() };
    Response::new(res)
}

impl Into<UserInfo> for User {
    fn into(self) -> UserInfo {
        UserInfo {
            user_id: self.user_id.to_string(),
            user_name: self.user_name,
            member_id: self.member_id.to_string(),
            member_type: self.member_type.to_string(),
            subscription_end_date: self.sub_end_date.to_string(),
            account_id: self.account_id.to_string(),
            account_balance: self.account_balance.to_f64().unwrap(),
            refer_code: self.refer_code,
            created_at: self.created_at.to_string(),
        }
    }
}