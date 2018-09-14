use super::{config, endpoints};
use actix::prelude::*;
use actix_web::{http, server, App};
use domain::actors::DomainExecutor;
use infra::connectors::ConnectionPoolBuilder;

pub struct AppState {
    pub domain: Addr<Syn, DomainExecutor>,
}

pub fn run(server_cfg: config::ServerConfig, pg_config: config::PostgresConfig) {
    let sys = actix::System::new("Plataforma Domain Writer Service");

    let domain_executor = {
        let pg_pool = ConnectionPoolBuilder::build_postgres(pg_config.0, pg_config.1, pg_config.2);

        SyncArbiter::start(3, move || DomainExecutor {
            postgres: pg_pool.clone(),
        })
    };

    server::new(move || {
        App::with_state(AppState {
            domain: domain_executor.clone(),
        }).scope("", |scope| {
            scope
                .resource("write", |r| {
                    r.method(http::Method::POST)
                        .with(endpoints::write_models)
                        .1
                        .limit(100000000);
                })
                .resource("schema/tables", |r| {
                    r.method(http::Method::POST).with(endpoints::create_table);
                })
        })
    }).bind(format!(
        "{addr}:{port}",
        addr = server_cfg.0,
        port = server_cfg.1
    ))
        .unwrap()
        .workers(server_cfg.2)
        .shutdown_timeout(1)
        .start();

    println!(
        "Started http server at {addr}:{port}",
        addr = server_cfg.0,
        port = server_cfg.1
    );

    let _ = sys.run();
}
