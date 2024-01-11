use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct AddBookRequest {
    #[validate(length(min = 1, message = "Book name Required"))]
    pub title: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateBookId {
    pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Book {
    pub uuid: String,
    pub title: String,
}

impl Book {
    pub fn new(uuid: String, title: String) -> Book {
        return Book { uuid, title };
    }
}
