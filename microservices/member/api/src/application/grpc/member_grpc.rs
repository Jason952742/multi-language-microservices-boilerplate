use std::str::FromStr;
use rust_decimal::prelude::ToPrimitive;
use sea_orm::ActiveEnum;
use tokio::sync::{mpsc, oneshot};
use tonic::{Code, Request, Response, Status};
use shared::{parse_code, to_uuid};
use crate::application::grpc::member_grpc::member_proto::{AddMemberRequest, MemberId, ListRequest, member_server, MemberInfo, MemberListReply, MemberReply, ProcessStatusReply, UpdateMemberRequest};
use crate::domain::commands::member_cmd::{MemberCommand, MemberEvent};
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::domain::entities::member;
use crate::domain::handlers::{MemberActor, run_member_actor};
use crate::domain::queries::member_qry::MemberQuery;

pub mod member_proto {
    tonic::include_proto!("member");
}

#[derive(Debug)]
pub struct MemberGrpc {
    tx: mpsc::Sender<MemberCommand>,
}

impl MemberGrpc {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        let actor = MemberActor::new(rx);
        tokio::spawn(run_member_actor(actor));
        Self { tx }
    }
}

#[tonic::async_trait]
impl member_server::Member for MemberGrpc {
    // #[tracing::instrument]
    async fn add_member(&self, request: Request<AddMemberRequest>) -> Result<Response<ProcessStatusReply>, Status> {
        let request = request.into_inner();
        tracing::info!("add member: {:?}", &request);

        let (resp_tx, resp_rx) = oneshot::channel();
        let command = MemberCommand::Create {
            id: to_uuid(&request.id),
            user_id: to_uuid(&request.user_id),
            user_name: request.user_name,
            resp: resp_tx,
        };
        if self.tx.send(command).await.is_err() {
            eprintln!("connection task shutdown");
        }
        match resp_rx.await.unwrap() {
            Ok(event) => match event {
                MemberEvent::Created { id } => Ok(to_process_reply(id.to_string())),
                _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
            },
            Err(e) => Err(e),
        }
    }

    // #[tracing::instrument]
    async fn disable_member(&self, request: Request<MemberId>) -> Result<Response<ProcessStatusReply>, Status> {
        let request = request.into_inner();
        tracing::info!("disable member: {:?}", &request);

        let (resp_tx, resp_rx) = oneshot::channel();
        let command = MemberCommand::Disable {
            id: to_uuid(&request.id),
            resp: resp_tx,
        };
        if self.tx.send(command).await.is_err() {
            eprintln!("connection task shutdown");
        }
        match resp_rx.await.unwrap() {
            Ok(event) => match event {
                MemberEvent::Disabled => Ok(to_process_reply(request.id)),
                _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
            },
            Err(e) => Err(e),
        }
    }

    // #[tracing::instrument]
    async fn update_member(&self, request: Request<UpdateMemberRequest>) -> Result<Response<ProcessStatusReply>, Status> {
        let request = request.into_inner();
        tracing::info!("update member: {:?}", &request);

        let (resp_tx, resp_rx) = oneshot::channel();
        let command = MemberCommand::Update {
            id: to_uuid(&request.id),
            member_type: MemberType::from_str(&request.member_type).unwrap(),
            level: request.level,
            active: request.active,
            description: request.description,
            resp: resp_tx,
        };
        if self.tx.send(command).await.is_err() {
            eprintln!("connection task shutdown");
        }
        match resp_rx.await.unwrap() {
            Ok(event) => match event {
                MemberEvent::Updated => Ok(to_process_reply(request.id)),
                _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
            },
            Err(e) => Err(e),
        }
    }

    // #[tracing::instrument]
    async fn get_members(&self, request: Request<ListRequest>) -> Result<Response<MemberListReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get member: {:?}", &request);

        let res = MemberQuery::get_members(
            request.page, request.per_page,
            request.status.map(|x| MemberStatus::from_str(&x).unwrap()),
            request.member_type.map(|x| MemberType::from_str(&x).unwrap()),
            request.level,
        ).await?;
        Ok(to_member_list_reply(res.0, res.1))
    }

    #[tracing::instrument]
    async fn get_member_by_user_id(&self, request: Request<MemberId>) -> Result<Response<MemberReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get members: {:?}", &request);

        match MemberQuery::get_member_by_id(to_uuid(&request.id)).await? {
            None => Err(Status::not_found(request.id)),
            Some(m) => Ok(to_member_reply(m))
        }
    }
}

fn to_process_reply(process_id: String) -> Response<ProcessStatusReply> {
    Response::new(ProcessStatusReply { code: parse_code(Code::Ok), message: "Processed".to_string(), success: true, process_id })
}

fn to_member_reply(model: member::Model) -> Response<MemberReply> {
    let r = MemberReply { code: parse_code(Code::Ok), message: "member".to_string(), data: Some(model.into()) };
    Response::new(r)
}

fn to_member_list_reply(models: Vec<member::Model>, num_pages: u64) -> Response<MemberListReply> {
    let list = models.clone().into_iter().map(|x| x.into()).collect();
    let r = MemberListReply { code: parse_code(Code::Ok), message: "member list".to_string(), data: list, num_pages };
    Response::new(r)
}

impl Into<MemberInfo> for member::Model {
    fn into(self) -> MemberInfo {
        MemberInfo {
            id: self.id.to_string(),
            user_id: self.user_id.to_string(),
            user_name: self.user_name,
            status: self.status.to_value(),
            member_type: self.member_type.to_value(),
            credit_score: self.credit_score.to_i32().unwrap(),
            point: self.point,
            level: self.level,
            active: self.active,
            description: self.description,
            created_at: self.created_at.to_string(),
        }
    }
}
