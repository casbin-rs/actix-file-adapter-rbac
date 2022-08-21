use casbin::{CoreApi, Enforcer, RbacApi};
use std::io;
use std::sync::RwLock;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

// simple handle
async fn success(enforcer: web::Data<RwLock<Enforcer>>, req: HttpRequest) -> HttpResponse {
    let mut e = enforcer.write().unwrap();
    println!("{:?}", req);
    assert_eq!(vec!["data2_admin"], e.get_roles_for_user("alice", None));

    HttpResponse::Ok().body("Success: alice is data2_admin.")
}

async fn fail(enforcer: web::Data<RwLock<Enforcer>>, req: HttpRequest) -> HttpResponse {
    let mut e = enforcer.write().unwrap();
    println!("{:?}", req);
    assert_eq!(vec!["data1_admin"], e.get_roles_for_user("alice", None));

    HttpResponse::Ok().body("Fail: alice is not data1_admin.") // In fact, it can't be displayed.
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    loge::init();

    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
