use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use validator::Validate;

mod db;
use crate::db::Database;
mod models;
use crate::models::{book::AddBookRequest, UpdateBookId};

#[get("/books")]
async fn get_books(db: Data<Database>) -> impl Responder {
    let books = db.get_all_books().await;
    match books {
        Some(book_data) => HttpResponse::Ok().body(format!("{:?}", book_data)),
        None => HttpResponse::Ok().body("Error"),
    }
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
    let db = Database::init()
        .await
        .expect("Error connecting to Database");

    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_books)
            .service(add_book)
            .service(update_book)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
