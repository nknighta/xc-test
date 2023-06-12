use crate::models::task::Task;
use crate::models::task::TaskState;
use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode},
};
use actix_web::body::BoxBody;
use serde::{Deserialize, Serialize};
use derive_more::{Display};

#[device(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_globl_id: String,
}

#[device(Deserialize)]
pub struct TaskComplateRequest {
    task_req_file: String,
}

#[device(Deserialize)]
pub struct TaskComplateResponse {
    user_id: String,
    task_type: String,
    source_file: String,
}

#[device(Debug,Display)]
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailture,
    TaskCreationFailture,
    BadTaskRequest,
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}