pub mod models;
pub mod schema;

use data_encoding::{HEXUPPER, HEXLOWER, HEXLOWER_PERMISSIVE, HEXUPPER_PERMISSIVE};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::BigInt;
use diesel::sql_types::Integer;
use diesel::{pg::PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use rand::distributions::{Distribution, Uniform};



pub fn updatePassword(conn: &mut PgConnection, pass: String, user: String) -> QueryResult<usize> {
    use schema::users::dsl::*;
    let encoded: String = HEXUPPER.encode(pass);
    let rows_inserted = diesel::insert_into(users)
    .values(&user, &encoded)
    .execute(conn);
    return rows_inserted;
}