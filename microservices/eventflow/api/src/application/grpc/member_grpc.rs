use std::str::FromStr;
use tokio::sync::{mpsc, oneshot};
use tonic::{Code, Request, Response, Status};
use shared::{parse_code, to_uuid};
use crate::application::grpc::member_grpc::refer_member_proto::{BindReferralRequest, Member, MemberListReply, MemberReply, ProcessStatusReply, refer_member_server, UpdateMemberRequest, UserIdRequest};
use crate::domain::commands::member_cmd::{MemberCommand, MemberEvent};
use crate::domain::entities::member;
use crate::domain::handlers::{MemberActor, run_member_actor};
use crate::domain::messages::MemberType;
use crate::domain::queries::member_qry::MemberQuery;

pub mod refer_member_proto {
    tonic::include_proto!("refer_member");
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
impl refer_member_server::ReferMember for MemberGrpc {

    #[tracing::instrument]
    async fn get_member_by_id(&self, request: Request<UserIdRequest>) -> Result<Response<MemberReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get member by id request: {:?}", &request);

        match MemberQuery::get_member_by_id(to_uuid(&request.id)).await? {
            None => Err(Status::not_found(request.id)),
            Some(m) => Ok(to_member_res(m))
        }
    }

    #[tracing::instrument]
    async fn get_my_referral(&self, request: Request<UserIdRequest>) -> Result<Response<MemberReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get my referral request: {:?}", &request);

        match MemberQuery::get_my_referral(to_uuid(&request.id)).await? {
            None => Err(Status::not_found(request.id)),
            Some(m) => Ok(to_member_res(m))
        }
    }

    #[tracing::instrument]
    async fn get_my_referees(&self, request: Request<UserIdRequest>) -> Result<Response<MemberListReply>, Status> {
        let request = request.into_inner();
        tracing::info!("get my referees request: {:?}", &request);

        let res = MemberQuery::get_my_referees(to_uuid(&request.id)).await?;
        Ok(to_member_list_res(res))
    }

    #[tracing::instrument]
    async fn update_member(&self, request: Request<UpdateMemberRequest>) -> Result<Response<ProcessStatusReply>, Status> {
        let request = request.into_inner();
        tracing::info!("update member  request: {:?}", &request);

        let (resp_tx, resp_rx) = oneshot::channel();
        let command = MemberCommand::Update {
            user_id: to_uuid(&request.user_id),
            member_type: MemberType::from_str(&request.member_type).unwrap(),
            level: request.level,
            active: request.active,
            description: request.description,
            resp: resp_tx
        };
        if self.tx.send(command).await.is_err() {
            eprintln!("connection task shutdown");
        }
        match resp_rx.await.unwrap() {
            Ok(event) => match event {
                MemberEvent::Updated => Ok(to_process_res(request.user_id.to_string())),
                _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
            },
            Err(e) => Err(e),
        }
    }

    #[tracing::instrument]
    async fn bind_referral(&self, request: Request<BindReferralRequest>) -> Result<Response<ProcessStatusReply>, Status> {
        let request = request.into_inner();
        tracing::info!("bind referral: {:?}", &request);

        let (resp_tx, resp_rx) = oneshot::channel();
        let command = MemberCommand::Bind {
            user_id: to_uuid(&request.user_id),
            referral_id: to_uuid(&request.referral_id),
            resp: resp_tx
        };
        if self.tx.send(command).await.is_err() {
            eprintln!("connection task shutdown");
        }
        match resp_rx.await.unwrap() {
            Ok(event) => match event {
                MemberEvent::Bound => Ok(to_process_res(request.user_id.to_string())),
                _ => Err(Status::failed_precondition(format!("error event {:?}", event))),
            },
            Err(e) => Err(e),
        }
    }
}

fn to_process_res(process_id: String) -> Response<ProcessStatusReply> {
    Response::new(ProcessStatusReply { code: parse_code(Code::Ok), message: "Processed".to_string(), success: true, process_id })
}

fn to_member_res(model: member::Model) -> Response<MemberReply> {
    let res = MemberReply { code: parse_code(Code::Ok), message: "member".to_string(), data: Some(model.into()) };
    Response::new(res)
}

fn to_member_list_res(models: Vec<member::Model>) -> Response<MemberListReply> {
    let list = models.clone().into_iter().map(|x| x.into()).collect();
    let res = MemberListReply { code: parse_code(Code::Ok), message: "member list".to_string(), data: list };
    Response::new(res)
}

impl Into<Member> for member::Model {
    fn into(self) -> Member {
        Member {
            user_id: self.user_id.to_string(),
            user_name: self.user_name,
            member_type: self.member_type.to_string(),
            level: self.level,
            hierarchy: self.hierarchy,
            active: self.active,
            description: self.description,
            created_at: self.created_at.to_string(),
        }
    }
}