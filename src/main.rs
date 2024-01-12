use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use uuid::Uuid;
use validator::Validate;

mod db;
use crate::{db::Database, models::book::Book};
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
async fn add_book(body: Json<AddBookRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let title = body.title.clone();
            let mut buffer = Uuid::encode_buffer();
            let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_book = db.add_book(Book::new(String::from(uuid), title)).await;

            match new_book {
                Some(created_book) => {
                    HttpResponse::Ok().body(format!("New Book Added : {:?}", created_book))
                }
                None => HttpResponse::Ok().body("Error : Can't add book"),
            }
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
