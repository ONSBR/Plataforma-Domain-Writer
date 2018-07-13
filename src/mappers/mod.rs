mod mapper;
mod redis_mapper;

use self::super::domain::models::ModelState;
pub use self::mapper::Mapper;
pub use self::redis_mapper::RedisMapper;
