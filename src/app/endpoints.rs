use super::server::AppState;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json};
use domain::messages::migration::{MigrationMessage};
use domain::messages::data::{CommitMessage};
use domain::models::migrations::{MigrationCommands};
use domain::models::schema::{Table};
use domain::models::data::{ModelContainer};
use futures::Future;


pub fn write_models(
    (req, data): (HttpRequest<AppState>, Json<Vec<ModelContainer>>),
    ) -> FutureResponse<HttpResponse> {
    req.state()
        .domain
        .send(CommitMessage { models: data.0 })
        .from_err()
        .and_then(|res| match res {
            Ok(ret) => Ok(HttpResponse::Ok().json(ret)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
    .responder()
}

pub fn create_table((req,data): (HttpRequest<AppState>, Json<Table>)) -> FutureResponse<HttpResponse> {
    req.state()
        .domain
        .send(MigrationMessage::<Table>{object: data.0, command: MigrationCommands::Create })
        .from_err()
        .and_then(|res| match res {
            Ok(data) => Ok(HttpResponse::Ok().json(data)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
    .responder()
}

