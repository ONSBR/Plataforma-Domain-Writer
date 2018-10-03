use actix::prelude::*;
use domain::models::data::{ModelContainer, ModelContainerBatchSql};
use domain::actors::DomainExecutor;
use infra::postgres_batch::BatchSql;

pub struct CommitMessage {
    pub models: Vec<ModelContainer>,
}

impl Message for CommitMessage {
    type Result = Result<String, String>;
}

impl Handler<CommitMessage> for DomainExecutor {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: CommitMessage, _: &mut Self::Context) -> Self::Result {
        for batch in BatchSql::from_containers(&msg.models) {
            if let Ok(conn) = self.postgres.get() {
                if let Err(_) = batch.execute(&conn) {
                    return Err(format!(
                        "Data could not be persisted. Error ocurred on model {}",
                        batch.table
                    ));
                }
            } else {
                return Err(
                    String::from(
                        "Data could not be persisted due to database connection issues. Please get in touch with an administrator.",));
            }
        }

        Ok(String::from("Data successfully persisted."))
    }
}
