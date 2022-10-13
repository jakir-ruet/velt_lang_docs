use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/v1", "../download"))
            .service(Files::new("/web", "../views"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
