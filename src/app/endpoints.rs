use super::server::AppState;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json};
use domain::messages::CommitMessage;
use domain::models::ModelContainer;
use futures::Future;

pub fn write_models(
    (req, data): (HttpRequest<AppState>, Json<Vec<ModelContainer>>),
) -> FutureResponse<HttpResponse> {
    req.state()
        .domain
        .send(CommitMessage { models: data.0 })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
