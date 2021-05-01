use std::error;
use std::fmt;
use std::result;

mod db;
mod todo;
mod todo_manager;

use db::Db;
use todo::manager_server::ManagerServer;
use todo_manager::TodoManager;

use dinglebit_config::{default_config, Config, MultiConfig};
use mongodb::{options::ClientOptions, Client};
use tonic::transport::Server;

#[derive(Debug)]
pub enum Error {
    NotFound,
    MongoDB(mongodb::error::Error),
    Oid(bson::oid::Error),
}

// Implement display so we can map to_string() as needed.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound => write!(f, "not found"),
            Error::MongoDB(e) => write!(f, "mongodb error: {}", e),
            Error::Oid(e) => write!(f, "oid error: {}", e),
        }
    }
}

#[tokio::main]
async fn main() -> result::Result<(), Box<dyn error::Error>> {
    // Our application configuration. We are just using default values
    // for now, but could eventually add a file and/or some other config.
    let cfg = MultiConfig::new(vec![default_config! {
        "mongo.uri" => "mongodb://localhost:27017",
        "mongo.db"  => "todo",
        "addr"      => "[::1]:50051"
    }]);

    // Make a connection to mongodb.
    let client_options = ClientOptions::parse(&cfg.string("mongo.uri")).await?;
    let client = Client::with_options(client_options)?;

    // Setup our handler with a connection to the database.
    let db = Db::new(client.database(&cfg.string("mongo.db")));
    let handler = TodoManager::new(db);

    // setup and start our protobuf server.
    let addr = cfg.string("addr").parse()?;
    Server::builder()
        .add_service(ManagerServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}
