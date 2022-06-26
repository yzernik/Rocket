#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

#[cfg(test)]
mod tests;

mod rusqlite;
mod sqlx;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(sqlx::stage())
        .attach(rusqlite::stage())
}
