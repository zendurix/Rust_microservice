use std::ops::Deref;

use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

type ConnPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_connection_pool(db_url: String) -> ConnPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    ConnPool::new(manager).expect("db Connection error")
}

pub struct DbConnection(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConnection, Self::Error> {
        let pool = request.guard::<State<ConnPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
