use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("index.html"))
}

async fn css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(include_str!("style.css"))
}

async fn js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(include_str!("script.js"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/style.css", web::get().to(css))
            .route("/script.js", web::get().to(js))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
