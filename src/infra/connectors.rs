use r2d2;
use r2d2_postgres;
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
            Err(_) => Err(()),
        }
    }

    pub fn build_postgres(
        host: String,
        port: u32,
        db: String,
    ) -> r2d2::Pool<r2d2_postgres::PostgresConnectionManager> {
        let connection_string = format!(
            "postgres://postgres:postgres@{host}:{port}/{db}",
            host = host,
            port = port,
            db = db
        );
        let manager = r2d2_postgres::PostgresConnectionManager::new(
            connection_string,
            r2d2_postgres::TlsMode::None,
        ).unwrap();

        match ConnectionPoolBuilder::build(manager, Some(32)) {
            Ok(pool) => pool,
            Err(_) => panic!(
                "Could not connect to postgres instance on {host}",
                host = host,
            ),
        }
    }
}
