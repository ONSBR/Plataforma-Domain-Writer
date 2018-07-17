use super::super::infra::postgres_batch::CommandTypes;

pub trait Mapper {
    type Result;

    fn get(&self, solution_id: &str, model_type: &str, stmt_type: CommandTypes) -> Option<String>;

    fn set(
        &self,
        solution_id: &str,
        model_type: &str,
        stmt_type: CommandTypes,
        query: String,
    ) -> Self::Result;
}
