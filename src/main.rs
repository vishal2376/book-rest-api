use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    App, HttpServer,
};
use uuid::Uuid;
use validator::Validate;

mod db;
use crate::{
    db::{pizza_data_trait::BookDataTrait, Database},
    errors::BookError,
    models::book::Book,
};
mod models;
use crate::models::{book::AddBookRequest, UpdateBookId};
mod errors;

#[get("/books")]
async fn get_books(db: Data<Database>) -> Result<Json<Vec<Book>>, BookError> {
    let books = Database::get_all_books(&db).await;
    match books {
        Some(book_found) => Ok(Json(book_found)),
        None => Err(BookError::NoBookFound),
    }
}

#[post("/books")]
async fn add_book(body: Json<AddBookRequest>, db: Data<Database>) -> Result<Json<Book>, BookError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let title = body.title.clone();
            let mut buffer = Uuid::encode_buffer();
            let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_book = Database::add_book(&db, Book::new(String::from(uuid), title)).await;

            match new_book {
                Some(created_book) => Ok(Json(created_book)),
                None => Err(BookError::BookCreationFailure),
            }
        }
        Err(_) => Err(BookError::BookCreationFailure),
    }
}

#[patch("/books/{uuid}")]
async fn update_book(
    book_id: Path<UpdateBookId>,
    body: Json<AddBookRequest>,
    db: Data<Database>,
) -> Result<Json<Book>, BookError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let title = body.title.clone();
            let uuid = book_id.into_inner().uuid;
            let update_result = Database::update_book(&db, uuid, title).await;

            match update_result {
                Some(updated_book) => Ok(Json(updated_book)),
                None => Err(BookError::NoBookFound),
            }
        }
        Err(_) => Err(BookError::BookCreationFailure),
    }
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
