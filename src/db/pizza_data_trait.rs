use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

use crate::models::Book;

use super::Database;

#[async_trait]
pub trait BookDataTrait {
    async fn get_all_books(db: &Data<Database>) -> Option<Vec<Book>>;
    async fn add_book(db: &Data<Database>, book: Book) -> Option<Book>;
    async fn update_book(db: &Data<Database>, uuid: String, title: String) -> Option<Book>;
}

#[async_trait]
impl BookDataTrait for Database {
    async fn get_all_books(db: &Data<Database>) -> Option<Vec<Book>> {
        let result = db.client.select("books_db").await;
        match result {
            Ok(all_books) => Some(all_books),
            Err(_) => None,
        }
    }
    async fn add_book(db: &Data<Database>, book: Book) -> Option<Book> {
        let new_book = db
            .client
            .create(("books_db", book.uuid.clone()))
            .content(book)
            .await;

        match new_book {
            Ok(created_book) => created_book,
            Err(_) => None,
        }
    }
    async fn update_book(db: &Data<Database>, uuid: String, title: String) -> Option<Book> {
        let find_book: Result<Option<Book>, Error> = db.client.select(("books_db", &uuid)).await;

        match find_book {
            Ok(found) => match found {
                Some(_book_found) => {
                    let update_book: Result<Option<Book>, Error> = db
                        .client
                        .update(("books_db", &uuid))
                        .merge(Book { uuid, title })
                        .await;

                    match update_book {
                        Ok(book) => book,
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }
}
