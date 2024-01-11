use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use validator::Validate;

mod models;
use crate::models::book::AddBookRequest;

#[get("/books")]
async fn get_books() -> impl Responder {
    HttpResponse::Ok().body("Books available")
}

#[post("/books")]
async fn add_book(body: Json<AddBookRequest>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let book_name = body.title.clone();
            HttpResponse::Ok().body(format!("Book Added: {}", book_name))
        }
        Err(_) => HttpResponse::Ok().body("Book title required."),
    }
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
