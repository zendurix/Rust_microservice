#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

pub mod db;
pub mod db_connection;
pub mod rest;
pub mod schema;

use rocket::{config::Environment, Config};

fn create_routes(db_url: String) {
    let config = Config::build(Environment::Staging)
        .address("localhost")
        .port(8000)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .manage(db_connection::init_connection_pool(db_url))
        .mount(
            "/book",
            routes![
                rest::get_all_books,
                rest::get_book,
                rest::insert_book,
                rest::edit_book,
                rest::delete_book,
            ],
        )
        .launch();
}

fn main() {
    let db_url = "_".to_string();

    create_routes(db_url);
}
