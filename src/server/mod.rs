mod handler;

use actix_web::{web, App, HttpServer, HttpResponse};
use diesel::{r2d2::{Pool, ConnectionManager}, mysql::MysqlConnection};
use dotenv::dotenv;
use std::env;
use handler::fraction_handler;

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                // .data(RequestContext::new())

                .service(handler::index)

                .service(
                    web::scope("/fraction")
                        .service(fraction_handler::add)
                )
        ).default_service(
            web::route().to(|| HttpResponse::NotFound().body("404 - Not Found"))
        )
    })
    .bind("127.0.0.1:8080")?
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