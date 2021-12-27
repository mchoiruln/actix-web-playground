use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port;
    match env::var("PORT") {
        Ok(val) => port = val,
        Err(_e) => port = "8080".to_string(),
    }
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Starting Web server {}", &port);
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default()).wrap(Logger::new("%a %{User-Agent}i"))
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:".to_owned()+&port)?
    .run()
    .await
}