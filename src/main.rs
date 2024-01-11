use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder};

#[get("/books")]
async fn get_books() -> impl Responder {
    HttpResponse::Ok().body("Books available")
}

#[post("/books")]
async fn add_book() -> impl Responder {
    HttpResponse::Ok().body("Book Added")
}

#[patch("/books/{uuid}")]
async fn update_book() -> impl Responder {
    HttpResponse::Ok().body("Book Updated")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_books)
            .service(add_book)
            .service(update_book)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
