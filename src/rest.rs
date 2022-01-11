use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::{
    db::{self, Book, InsertableBook},
    db_connection::DbConnection,
};

#[get("/")]
pub fn get_all_books(connection: DbConnection) -> Result<Json<Vec<Book>>, Status> {
    db::get_all_books(&connection)
        .map(|ma| Json(ma))
        .map_err(|error| error_status(error))
}

#[get("/<book_id>")]
pub fn get_book(book_id: String, connection: DbConnection) -> Result<Json<Book>, Status> {
    db::get_book(book_id.parse().expect("Wrong Id error"), &connection)
        .map(|b| Json(b))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<book>")]
pub fn insert_book(
    book: Json<InsertableBook>,
    connection: DbConnection,
) -> Result<status::Created<Json<Book>>, Status> {
    db::insert_book(book.0, &connection)
        .map(|book| book_created(book))
        .map_err(|error| error_status(error))
}

#[post("/edit", format = "application/json", data = "<book>")]
pub fn edit_book(
    book: Json<Book>,
    connection: DbConnection,
) -> Result<status::Created<Json<Book>>, Status> {
    db::update_book(book.0, &connection)
        .map(|book| book_created(book))
        .map_err(|error| error_status(error))
}

#[get("/delete/<book_id>")]
pub fn delete_book(book_id: String, connection: DbConnection) -> Result<Json<Book>, Status> {
    db::delete_book(book_id.parse().expect("Wrong Id error"), &connection)
        .map(|b| Json(b))
        .map_err(|error| error_status(error))
}

fn book_created(book: Book) -> status::Created<Json<Book>> {
    status::Created(
        format!("localhost:8000/book/{id}", id = book.id).to_string(),
        Some(Json(book)),
    )
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
