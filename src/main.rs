extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_redis;
extern crate redis;
extern crate time;
extern crate uuid;
extern crate barrel;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod app;
mod domain;
mod infra;
mod mappers;

fn main() {
    //TODO: get settings from env
    let localhost = String::from("127.0.0.1");
    let server_cfg = app::config::ServerConfig(localhost.clone(), 8000, 3);
    let pg_cfg = app::config::PostgresConfig(localhost.clone(), 5432, String::from("postgres"));

    app::server::run(server_cfg, pg_cfg);
}
