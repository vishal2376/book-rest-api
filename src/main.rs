use actix_web::{
    get, patch, post,
    web::{Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use validator::Validate;

mod models;
use crate::models::{book::AddBookRequest, UpdateBookId};

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
async fn update_book(book_id: Path<UpdateBookId>) -> impl Responder {
    let uuid = book_id.into_inner().uuid;
    HttpResponse::Ok().body(format!("Book Id Updated : {}", uuid))
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
