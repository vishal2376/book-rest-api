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
