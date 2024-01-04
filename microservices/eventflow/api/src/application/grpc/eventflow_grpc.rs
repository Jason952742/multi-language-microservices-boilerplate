use std::str::FromStr;
use tokio::sync::{mpsc, oneshot};
use tonic::{Code, Request, Response, Status};
use shared::{parse_code, to_uuid};
use crate::application::grpc::eventflow_grpc::eventflow_proto::{AccountTransaction, AccountTransactionReply, eventflow_server, ListRequest, MemberSubscriptionReply, MemberSubscriptionRequest, TransactionId, TransactionListReply, TransactionReply, UserCreatedReply, UserCreateRequest};
use crate::domain::commands::eventflow_cmd::{EventflowCommand, EventflowEvent};
use crate::domain::handlers::{EventflowActor, run_eventflow_actor};

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

    #[tracing::instrument]
    async fn get_transaction_by_id(&self, request: Request<TransactionId>) -> Result<Response<TransactionReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get transaction by id: {:?}", &request);

        // match MemberQuery::get_member_by_id(to_uuid(&request.id)).await? {
        //     None => Err(Status::not_found(request.id)),
        //     Some(m) => Ok(to_member_res(m))
        // }

        todo!()
    }

    #[tracing::instrument]
    async fn get_transactions(&self, request: Request<ListRequest>) -> Result<Response<TransactionListReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get transactions: {:?}", &request);

        // match MemberQuery::get_my_referral(to_uuid(&request.id)).await? {
        //     None => Err(Status::not_found(request.id)),
        //     Some(m) => Ok(to_member_res(m))
        // }
        todo!()
    }

    #[tracing::instrument]
    async fn user_create(&self, request: Request<UserCreateRequest>) -> Result<Response<UserCreatedReply>, Status> {
        let request = request.into_inner();
        tracing::info!("user create: {:?}", &request);

        // let res = MemberQuery::get_my_referees(to_uuid(&request.id)).await?;
        // Ok(to_member_list_res(res))
        todo!()
    }

    #[tracing::instrument]
    async fn account_deposit(&self, request: Request<AccountTransaction>) -> Result<Response<AccountTransactionReply>, Status> {
        let request = request.into_inner();
        tracing::info!("account deposit: {:?}", &request);

        // let (resp_tx, resp_rx) = oneshot::channel();
        // let command = MemberCommand::Update {
        //     user_id: to_uuid(&request.user_id),
        //     member_type: MemberType::from_str(&request.member_type).unwrap(),
        //     level: request.level,
        //     active: request.active,
        //     description: request.description,
        //     resp: resp_tx
        // };
        // if self.tx.send(command).await.is_err() {
        //     eprintln!("connection task shutdown");
        // }
        // match resp_rx.await.unwrap() {
        //     Ok(event) => match event {
        //         MemberEvent::Updated => Ok(to_process_res(request.user_id.to_string())),
        //         _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
        //     },
        //     Err(e) => Err(e),
        // }

        todo!()
    }

    #[tracing::instrument]
    async fn account_withdraw(&self, request: Request<AccountTransaction>) -> Result<Response<AccountTransactionReply>, Status> {
        let request = request.into_inner();
        tracing::info!("account withdraw: {:?}", &request);

        // let (resp_tx, resp_rx) = oneshot::channel();
        // let command = MemberCommand::Bind {
        //     user_id: to_uuid(&request.user_id),
        //     referral_id: to_uuid(&request.referral_id),
        //     resp: resp_tx
        // };
        // if self.tx.send(command).await.is_err() {
        //     eprintln!("connection task shutdown");
        // }
        // match resp_rx.await.unwrap() {
        //     Ok(event) => match event {
        //         MemberEvent::Bound => Ok(to_process_res(request.user_id.to_string())),
        //         _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
        //     },
        //     Err(e) => Err(e),
        // }
        todo!()
    }

    #[tracing::instrument]
    async fn member_subscription(&self, request: Request<MemberSubscriptionRequest>) -> Result<Response<MemberSubscriptionReply>, Status> {
        let request = request.into_inner();
        tracing::info!("member subscription: {:?}", &request);

        todo!()
    }
}

// fn to_process_res(process_id: String) -> Response<ProcessStatusReply> {
//     Response::new(ProcessStatusReply { code: parse_code(Code::Ok), message: "Processed".to_string(), success: true, process_id })
// }
//
// fn to_member_res(model: member::Model) -> Response<MemberReply> {
//     let res = MemberReply { code: parse_code(Code::Ok), message: "member".to_string(), data: Some(model.into()) };
//     Response::new(res)
// }
//
// fn to_member_list_res(models: Vec<member::Model>) -> Response<MemberListReply> {
//     let list = models.clone().into_iter().map(|x| x.into()).collect();
//     let res = MemberListReply { code: parse_code(Code::Ok), message: "member list".to_string(), data: list };
//     Response::new(res)
// }
//
// impl Into<Member> for member::Model {
//     fn into(self) -> Member {
//         Member {
//             user_id: self.user_id.to_string(),
//             user_name: self.user_name,
//             member_type: self.member_type.to_string(),
//             level: self.level,
//             hierarchy: self.hierarchy,
//             active: self.active,
//             description: self.description,
//             created_at: self.created_at.to_string(),
//         }
//     }
// }