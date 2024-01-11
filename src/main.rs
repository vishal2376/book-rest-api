use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/books")]
async fn get_books() -> impl Responder {
    HttpResponse::Ok().body("Books available")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_books))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
