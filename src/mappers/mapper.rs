use super::ModelState;

pub trait Mapper {
    type Result;

    fn get(&self, solution_id: &str, model_type: &str, stmt_type: ModelState) -> Option<String>;

    fn set(
        &self,
        solution_id: &str,
        model_type: &str,
        stmt_type: ModelState,
        query: String,
    ) -> Self::Result;
}
