use super::schema::book;
use diesel::{
    AsChangeset, ExpressionMethods, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "book"]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub author_name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "book"]
pub struct InsertableBook {
    pub name: String,
    pub author_name: String,
}

pub fn get_all_books(connection: &PgConnection) -> QueryResult<Vec<Book>> {
    book::table
        .select(book::all_columns)
        .load::<Book>(connection)
}

pub fn get_book(id: i32, connection: &PgConnection) -> QueryResult<Book> {
    book::table.find(id).get_result::<Book>(connection)
}

pub fn insert_book(book: InsertableBook, connection: &PgConnection) -> QueryResult<Book> {
    diesel::insert_into(book::table)
        .values(&book)
        .get_result(connection)
}

pub fn update_book(updated_book: Book, connection: &PgConnection) -> QueryResult<Book> {
    diesel::update(book::table.find(updated_book.id))
        .set((
            book::name.eq(updated_book.name),
            book::author_name.eq(updated_book.author_name),
        ))
        .get_result(connection)
}

pub fn delete_book(id: i32, connection: &PgConnection) -> QueryResult<Book> {
    diesel::delete(book::table.find(id)).get_result::<Book>(connection)
}
