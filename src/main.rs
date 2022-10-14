use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/docs/{n}")]
async fn docs(n: web::Path<String>) -> impl Responder {
    let name = n.into_inner();

    if name == "hello_world.html" {
        return HttpResponse::Ok().body(format!(
            "{}{}",
            include_str!("../views/hello_world.html"),
            include_str!("../views/docs.html")
        ));
    }
    if name == "variables_&_specifier.html" {
        return HttpResponse::Ok().body(format!(
            "{}{}",
            include_str!("../views/variables_&_specifier.html"),
            include_str!("../views/docs.html")
        ));
    }
    if name == "getting_started.html" {
        return HttpResponse::Ok().body(format!(
            "{}{}",
            include_str!("../views/getting_started.html"),
            include_str!("../views/docs.html")
        ));
    } else {
        HttpResponse::Ok().body(include_str!("../views/docs.html"))
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(docs)
            .service(Files::new("/v1", "../download"))
            .service(Files::new("/web", "../views"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
