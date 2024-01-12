use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error, Surreal,
};

use crate::models::book::Book;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("surreal").use_db("books_db").await.unwrap();

        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("books_db"),
        })
    }

    pub async fn get_all_books(&self) -> Option<Vec<Book>> {
        let result = self.client.select("books_db").await;
        match result {
            Ok(all_books) => Some(all_books),
            Err(_) => None,
        }
    }

    pub async fn add_book(&self, book: Book) -> Option<Book> {
        let new_book = self
            .client
            .create(("books_db", book.uuid.clone()))
            .content(book)
            .await;

        match new_book {
            Ok(created_book) => created_book,
            Err(_) => None,
        }
    }
}
