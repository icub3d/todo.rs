use crate::db::Db;
use crate::Error;

use crate::todo::{
    manager_server::Manager, AllRequest, AllResponse, CreateRequest, DeleteRequest, DeleteResponse,
    GetRequest, List, UpdateRequest,
};
use tonic::{Code, Request, Response, Status};

pub type TonicResult<T> = std::result::Result<Response<T>, Status>;

// Helper macro, this turned out to be a common Result handler where
// we translate mongodb errors to tonic errors.
macro_rules! tonic_result {
    ($result:expr, $ok:ident, $resp:expr) => {
        match $result {
            Ok($ok) => Ok(Response::new($resp)),
            Err(e) => match e {
                Error::NotFound => Err(Status::new(Code::NotFound, "list not found")),
                Error::Oid(e) => Err(Status::new(
                    Code::InvalidArgument,
                    format!("invalid id: {}", e.to_string()),
                )),
                Error::MongoDB(e) => Err(Status::new(Code::Unknown, e.to_string())),
            },
        }
    };
}

// This will handle our protobuf service.
pub struct TodoManager {
    db: Db,
}

impl TodoManager {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl Manager for TodoManager {
    async fn all(&self, _: Request<AllRequest>) -> TonicResult<AllResponse> {
        let ll = self
            .db
            .all()
            .await
            .map_err(|e| Status::new(Code::Unknown, e.to_string()))?;
        Ok(Response::new(AllResponse { lists: ll }))
    }

    async fn create(&self, request: Request<CreateRequest>) -> TonicResult<List> {
        let l = self
            .db
            .create(&request.into_inner().name)
            .await
            .map_err(|e| Status::new(Code::Unknown, e.to_string()))?;
        Ok(Response::new(l))
    }

    async fn get(&self, request: Request<GetRequest>) -> TonicResult<List> {
        tonic_result!(self.db.get(&request.into_inner().id).await, l, l)
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> TonicResult<DeleteResponse> {
        let resp = DeleteResponse {};

        tonic_result!(self.db.delete(&request.into_inner().id).await, _l, resp)
    }

    async fn update(&self, request: Request<UpdateRequest>) -> TonicResult<List> {
        let list = request
            .into_inner()
            .list
            .ok_or(Status::new(Code::InvalidArgument, "list required"))?;

        tonic_result!(self.db.update(list.clone()).await, _l, list)
    }
}
