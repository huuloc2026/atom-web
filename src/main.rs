use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world! This is a simple Actix-web server. From Jake Onyx")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server đang chạy tại http://127.0.0.1:8080");


    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}