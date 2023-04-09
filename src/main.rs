#[macro_use] extern crate rocket;

mod auth;
use auth::{BasicAuth};

use rocket::serde::json::{Value, json};
use rocket::response::{status};



#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([{"id": 1, "name": "John Doe"}, {"id": 2, "name":"Sasha Lee"}])
}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32) -> Value {
    json!({"id": id, "name": "John Doe"})
}

#[post("/rustacean", format = "json")]
fn create_rustaceans() -> Value {
    json!({"id": 1, "name": "John Doe"})
}

#[put("/rustaceans", format = "json")]
fn update_rustaceans() -> Value {
    json!({"id": 1, "name": "John Doe"})
}


#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn not_auth() -> Value {
    json!("Not authorized")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        get_rustaceans,
        view_rustaceans,
        create_rustaceans,
        update_rustaceans,
        delete_rustacean
    ])
    .register("/", catchers![
        not_found,
        not_auth
    ])
}
