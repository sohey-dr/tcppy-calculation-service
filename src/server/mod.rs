mod handler;

use actix_web::{web, http, App, HttpServer};
use actix_cors::Cors;
use diesel::{r2d2::{Pool, ConnectionManager}, mysql::MysqlConnection};
use dotenv::dotenv;
use std::env;
use handler::fraction_handler;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    HttpServer::new(|| {
        App::new().wrap(
            Cors::default().supports_credentials()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE, http::header::ORIGIN])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
        ).service(
            web::scope("/api")
                // .data(RequestContext::new())

                .service(handler::index)

                .service(
                    web::scope("/fraction")
                        .service(fraction_handler::addition)
                )
        ).default_service(
            web::route().to(handler::not_found)
        )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

// NOTE: 以下の実装はDBを使用した際に使用すること
#[warn(dead_code)]
#[derive(Clone)]
pub struct RequestContext {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl RequestContext {
    #[warn(dead_code)]
    pub fn new() -> RequestContext {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");

        RequestContext { pool }
    }
}