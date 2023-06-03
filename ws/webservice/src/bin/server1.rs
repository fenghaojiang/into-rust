
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

pub fn general_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/heath", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web service is running")
}


// run http server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(general_router);


    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}







