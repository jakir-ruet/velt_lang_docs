
use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use actix_files::Files;

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../views/index.html"))
}

#[get("/install")]
async fn install() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../views/quick_start.html"))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "../../velt/target/debug"))
            .service(greet)
            .service(install)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
