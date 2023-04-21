#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{CreateRustocean, Rustocean};
use rocket::response::status;
use rocket::serde::json::{json, Value, Json};
use rocket_sync_db_pools::database;
use schema::rustoceans;

#[database("sqlite")]
struct DbCon(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbCon) -> Value {
    db.run(|c| {
        let r = rustoceans::table
            .order(rustoceans::id.desc())
            .limit(100)
            .load::<Rustocean>(c)
            .expect("DB Error occurred");

        json!(r)
    })
    .await
}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32) -> Value {
    json!({"id": id, "name": "John Doe"})
}

#[post("/rustaceans", format = "json", data="<create_dto>")]
async fn create_rustaceans(_auth: BasicAuth, db: DbCon, create_dto: Json<CreateRustocean>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustoceans::table)
            .values(create_dto.into_inner())
            .execute(c)
            .expect("Error to add create_dto");

        json!(result)
    }).await
}

#[put("/rustaceans", format = "json")]
fn update_rustaceans(_auth: BasicAuth) -> Value {
    json!({"id": 1, "name": "John Doe"})
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("404 Not found!")
}

#[catch(401)]
fn not_auth() -> Value {
    json!("401 Not authorized!")
}

#[catch(403)]
fn forbidden() -> Value {
    json!("403 Forbidden!")
}

#[catch(422)]
fn wrong_params() -> Value {
    json!("422 Wrong params")
}

#[catch(400)]
fn bad_request() -> Value {
    json!("400 Bad request")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustaceans,
                create_rustaceans,
                update_rustaceans,
                delete_rustacean
            ],
        )
        .attach(DbCon::fairing())
        .register("/", catchers![not_found, not_auth, forbidden, wrong_params, bad_request])
}
