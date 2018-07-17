use super::super::infra::postgres_batch::CommandTypes;
use super::Mapper;
use r2d2;
use r2d2_redis;
use redis;
use redis::Commands;

pub struct RedisMapper {
    pool: r2d2::Pool<r2d2_redis::RedisConnectionManager>,
}

impl RedisMapper {
    pub fn new(pool: r2d2::Pool<r2d2_redis::RedisConnectionManager>) -> Self {
        Self { pool }
    }
}

impl Mapper for RedisMapper {
    type Result = redis::RedisResult<()>;

    fn get(&self, solution_id: &str, map_id: &str, stmt_type: CommandTypes) -> Option<String> {
        let conn = self.pool.get().unwrap();
        let key = format!("{}_{}_{}", solution_id, map_id, stmt_type);

        match conn.get(key) {
            Ok(k) => Some(k),
            Err(_) => None,
        }
    }

    fn set(
        &self,
        solution_id: &str,
        model_type: &str,
        stmt_type: CommandTypes,
        query: String,
    ) -> Self::Result {
        let key: String = format!("{}_{}_{}", solution_id, model_type, stmt_type);
        let conn = self.pool.get().unwrap();
        let _: () = try!(conn.set(key, query));
        Ok(())
    }
}
