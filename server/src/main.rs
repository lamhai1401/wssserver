#[macro_use]
extern crate log;
use actix::Actor;
use actix_cors::Cors;
use actix_rt;
use actix_web::{http, middleware::Logger, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;
// internal pkg
mod routes;
mod websocket;

// the cfg(test) attribute so this module is only compiled in tests.
#[cfg(test)]
mod tests;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    // at the top of main()
    let pool = db::new_pool();
    let server = websocket::Server::new().start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&env::var("CLIENT_HOST").unwrap())
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .data(pool.clone()) // share db connection pool to app
            .data(server.clone())
            // .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(routes::routes)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
