
use std::io;

use actix_web::{web, App, HttpResponse, HttpServer};


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    loge::init();

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
