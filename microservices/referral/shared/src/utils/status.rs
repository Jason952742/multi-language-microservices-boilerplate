use async_nats::RequestErrorKind;
use neo4rs::Error;
use sea_orm::{DbErr, TransactionError};
use tonic::metadata::MetadataMap;
use tonic::{Code, Status};
use tonic_types::{ErrorDetails, StatusExt};
use uuid::Uuid;

pub struct GrpcStatusTool;

impl GrpcStatusTool {
    pub fn invalid(e: &str) -> Status {
        Status::with_error_details_and_metadata(
            Code::InvalidArgument,
            "Invalid Argument",
            ErrorDetails::with_bad_request_violation("error", format!("{:?}", e)),
            MetadataMap::new(),
        )
    }

    pub fn no_found(id: Uuid) -> Status {
        Status::with_error_details_and_metadata(
            Code::NotFound,
            "Not Found",
            ErrorDetails::with_bad_request_violation("error", format!("{:?} not found", id)),
            MetadataMap::new(),
        )
    }

    pub fn tr_error(e: TransactionError<DbErr>) -> Status {
        Status::with_error_details_and_metadata(
            Code::FailedPrecondition,
            "Database Transaction Error",
            ErrorDetails::with_bad_request_violation("error", e.to_string()),
            MetadataMap::new(),
        )
    }

    pub fn un_authenticated(identifier: &str) -> Status {
        Status::with_error_details_and_metadata(
            Code::Unauthenticated,
            "Un Authenticated",
            ErrorDetails::with_bad_request_violation("error", format!("{:?} Unauthenticated", identifier)),
            MetadataMap::new(),
        )
    }

    pub fn nats_error(err_kind: RequestErrorKind) -> Status {
        match err_kind {
            RequestErrorKind::TimedOut => Status::failed_precondition("nats timed out"),
            RequestErrorKind::NoResponders => Status::not_found("nats no found"),
            RequestErrorKind::Other => Status::internal("nats error"),
        }
    }

    pub fn db_error(e: DbErr) -> Status {
        Status::with_error_details_and_metadata(
            Code::FailedPrecondition,
            "Database Error",
            ErrorDetails::with_bad_request_violation("error", e.to_string()),
            MetadataMap::new(),
        )
    }

    pub fn neo4j_error(err: neo4rs::Error) -> Status {
        match err {
            Error::IOError { .. } => Status::data_loss("neo4j io error"),
            Error::UrlParseError(_) => Status::invalid_argument("neo4j url parse error"),
            Error::UnsupportedScheme(_) => Status::invalid_argument("neo4j unsupported scheme"),
            Error::InvalidDnsName(_) => Status::invalid_argument("neo4j invalid dns name"),
            Error::ConnectionError => Status::internal("neo4j connection error"),
            Error::StringTooLong => Status::internal("neo4j string too long"),
            Error::MapTooBig => Status::internal("neo4j map too big"),
            Error::BytesTooBig => Status::internal("neo4j bytes too big"),
            Error::ListTooLong => Status::internal("neo4j list too long"),
            Error::InvalidConfig => Status::internal("neo4j invalid config"),
            Error::UnsupportedVersion(_) => Status::internal("neo4j unsupported version"),
            Error::UnexpectedMessage(_) => Status::invalid_argument("neo4j unexpected message"),
            Error::UnknownType(_) => Status::invalid_argument("neo4j unknown type"),
            Error::UnknownMessage(_) => Status::invalid_argument("neo4j unknown message"),
            Error::ConversionError => Status::invalid_argument("neo4j conversion error"),
            Error::AuthenticationError(_) => Status::unauthenticated("neo4j authentication error"),
            Error::InvalidTypeMarker(_) => Status::invalid_argument("neo4j invalid type marker"),
            Error::DeserializationError(_) => Status::failed_precondition("neo4j deserialization error"),
        }
    }
}

pub fn parse_code(code: Code) -> String {
    match code {
        Code::Ok => String::from("Ok"),
        Code::Cancelled => String::from("Cancelled"),
        Code::Unknown => String::from("Unknown"),
        Code::InvalidArgument => String::from("InvalidArgument"),
        Code::DeadlineExceeded => String::from("DeadlineExceeded"),
        Code::NotFound => String::from("NotFound"),
        Code::AlreadyExists => String::from("AlreadyExists"),
        Code::PermissionDenied => String::from("PermissionDenied"),
        Code::ResourceExhausted => String::from("ResourceExhausted"),
        Code::FailedPrecondition => String::from("FailedPrecondition"),
        Code::Aborted => String::from("Aborted"),
        Code::OutOfRange => String::from("OutOfRange"),
        Code::Unimplemented => String::from("Unimplemented"),
        Code::Internal => String::from("Internal"),
        Code::Unavailable => String::from("Unavailable"),
        Code::DataLoss => String::from("DataLoss"),
        Code::Unauthenticated => String::from("Unauthenticated"),
    }
}