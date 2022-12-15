#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use lazy_static::lazy_static;
use rocket::{Build, Rocket};

#[derive(FromForm, Debug)]
struct Filters {
    age: u8,
    active: bool,
}

#[derive(Debug)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    grade: u8,
    active: bool,
}

lazy_static! {
    static ref USERS: HashMap<&'static str, User> = {
        let mut map = HashMap::new();
        let uuid = "3e3dd4ae-3c37-40c6-aa64-7061f284ce28";
        map.insert(
            uuid,
            User {
                uuid: uuid.into(),
                name: "Benjamin".into(),
                age: 41,
                grade: 16,
                active: true,
            },
        );
        map
    };
}

#[route(GET, uri = "/user/<uuid>", rank = 1, format = "text/plain")]
fn user(uuid: &str) -> String
{
    let user = USERS.get(uuid);
    match user {
        Some(u) => format!("Found user: {:?}", u),
        None => String::from("User not found"),
    }
}

#[route(GET, uri = "/users/<grade>?<filters..>")]
fn users(grade: u8, filters: Filters)
{

}



#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![user, users])
}

