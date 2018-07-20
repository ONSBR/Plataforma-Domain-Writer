use r2d2;
use r2d2_postgres;
use r2d2_redis;
pub struct ConnectionPoolBuilder {}

impl ConnectionPoolBuilder {
    fn build<T: r2d2::ManageConnection>(
        manager: T,
        max_size: Option<u32>,
    ) -> Result<r2d2::Pool<T>, ()> {
        match r2d2::Pool::builder()
            .max_size(max_size.unwrap_or(10))
            .build(manager)
        {
            Ok(pool) => Ok(pool),
            Err(e) => Err(),
        }
    }

    pub fn build_redis(host: String, db: u32) -> r2d2::Pool<r2d2_redis::RedisConnectionManager> {
        let connection_string = format!{"redis://{}/{}", host, db};
        let manager = r2d2_redis::RedisConnectionManager::new(connection_string.as_str()).unwrap();
        match ConnectionPoolBuilder::build(manager, None) {
            Ok(pool) => pool,
            Err() => panic!("Could not connect to redis instance on {}/{}", host, db),
        }
    }

    pub fn build_postgres(
        host: String,
        port: u32,
        db: String,
    ) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager> {
        let connection_string = format!(
            "postgres://postgres@{host}:{port}/{db}",
            host = host,
            port = port,
            db = db
        );
        let manager = r2d2_postgres::PostgresConnectionManager::new(
            connection_string,
            r2d2_postgres::TlsMode::None,
        ).unwrap();

        ConnectionPoolBuilder::build(manager, Some(32))
    }
}
