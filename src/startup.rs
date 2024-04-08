use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use sqlx::{PgConnection, PgPool};
use tracing_actix_web::TracingLogger;
use crate::routes::{health_check, subscribe};


//Notice the different signature!
//We return `Server` on the happy path, and we dropped the`async` keyword
//We have no .await call, so it is not needed anymore.
pub fn run(
    tcp_listener: TcpListener,
    db_pool: PgPool,
)-> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // instead of Logger::default
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
        .listen(tcp_listener)?
        .run();
    Ok(server)
}