#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::io::Cursor;

use lazy_static::lazy_static;

use rocket::{Build, Rocket, request::FromParam};
use rocket::http::ContentType;
use rocket::response::{self, Responder, Response};

#[derive(FromForm, Debug)]
struct Filters {
    age: u8,
    active: bool,
}

struct NameGrade<'r> {
    name: &'r str,
    grade: u8,
}

impl<'r> FromParam<'r> for NameGrade<'r> {
    type Error = &'static str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error>
    {
        const ERROR_MESSAGE: Result<NameGrade, &'static str> =
            Err("Error parsing user parameter");
        let name_grade_vec: Vec<&'r str> = param.split('_').collect();
        match name_grade_vec.len() {
            2 => match name_grade_vec[1].parse::<u8>() {
                Ok(n) => Ok(Self {
                    name: name_grade_vec[0],
                    grade: n,
                }),
                Err(_) => ERROR_MESSAGE,
            },
            _ => ERROR_MESSAGE
        }
    }
}

#[derive(Debug)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    grade: u8,
    active: bool,
}

impl<'r> Responder<'r, 'r> for &'r User {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'r>
    {
        let user = format!("Found user: {:?}", self);
        Response::build()
            .sized_body(user.len(), Cursor::new(user))
            .raw_header("X-USER-ID", self.uuid.to_string())
            .header(ContentType::Plain)
            .ok()
    }
}

lazy_static! {
    static ref USERS: HashMap<&'static str, User> = {
        let mut map = HashMap::new();
        map.insert("9f35b47d-d3b3-4f91-b96e-27a59d1db40f", User { uuid: "9f35b47d-d3b3-4f91-b96e-27a59d1db40f".into(), name: "Sarah Smith".into(), age: 23, grade: 12, active: true, }, );
        map.insert("5e5f9b9c-6cd9-4efc-ad08-ab6c8b946921", User { uuid: "5e5f9b9c-6cd9-4efc-ad08-ab6c8b946921".into(), name: "John Doe".into(), age: 29, grade: 9, active: false, }, );
        map.insert("7bfd3f14-c749-4b9e-907a-a1f01d23b72c", User { uuid: "7bfd3f14-c749-4b9e-907a-a1f01d23b72c".into(), name: "Jessica Williams".into(), age: 25, grade: 11, active: true, }, );
        map.insert("2d7e45e3-3c7b-4868-b3b7-f9da958b8053", User { uuid: "2d7e45e3-3c7b-4868-b3b7-f9da958b8053".into(), name: "Michael Brown".into(), age: 35, grade: 9, active: true, }, );
        map.insert("ad1f91c4-5ceb-4fd5-b8cc-f59d1a4a4b5b", User { uuid: "ad1f91c4-5ceb-4fd5-b8cc-f59d1a4a4b5b".into(), name: "Emily Johnson".into(), age: 20, grade: 10, active: false, }, );
        map.insert("547c3b3d-b3de-4d5b-8287-ed33a6f5a2c1", User { uuid: "547c3b3d-b3de-4d5b-8287-ed33a6f5a2c1".into(), name: "Christopher Rodriguez".into(), age: 26, grade: 12, active: true, }, );
        map.insert("f63f1491-6b8d-4f20-a9ef-a77d6f838c2b", User { uuid: "f63f1491-6b8d-4f20-a9ef-a77d6f838c2b".into(), name: "Stephanie Kim".into(), age: 24, grade: 11, active: false, }, );
        map.insert("ed2514f8-a541-4e05-9aa3-a3a9d3b54736", User { uuid: "ed2514f8-a541-4e05-9aa3-a3a9d3b54736".into(), name: "Daniel Davis".into(), age: 30, grade: 9, active: true, }, );
        map.insert("6d50f6a3-f746-4f75-b51c-d2744fd7da8b", User { uuid: "6d50f6a3-f746-4f75-b51c-d2744fd7da8b".into(), name: "Ashley Thompson".into(), age: 21, grade: 10, active: true, }, );
        map.insert("a9c5b3f7-9a1a-4b2c-abcf-16f6a2d38a61", User { uuid: "a9c5b3f7-9a1a-4b2c-abcf-16f6a2d38a61".into(), name: "Matthew Martin".into(), age: 27, grade: 12, active: false, }, );
        map.insert("cb3d3b7e-fba3-4d0c-9c61-7b45c3b1dbe2", User { uuid: "cb3d3b7e-fba3-4d0c-9c61-7b45c3b1dbe2".into(), name: "Samantha Lee".into(), age: 25, grade: 11, active: true, }, );
        map.insert("f5e5e5b3-f3b3-4d91-a96e-27a59d1db40f", User { uuid: "f5e5e5b3-f3b3-4d91-a96e-27a59d1db40f".into(), name: "William Smith".into(), age: 29, grade: 9, active: false, }, );
        map.insert("4e5f9b9c-6cd9-4efc-ad08-ab6c8b946921", User { uuid: "4e5f9b9c-6cd9-4efc-ad08-ab6c8b946921".into(), name: "Hannah Doe".into(), age: 23, grade: 12, active: true, }, );
        map
    };
}

struct NewUser<'a>(Vec<&'a User>);

impl<'r> Responder<'r, 'r> for NewUser<'r> {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'r>
    {
        let user = self.0.iter()
            .map(|u| format!("{:?}", u))
            .collect::<Vec<String>>()
            .join(",");
        Response::build()
            .sized_body(user.len(), Cursor::new(user))
            .header(ContentType::Plain)
            .ok()
    }
}


#[route(GET, uri = "/user/<uuid>", rank = 1, format = "text/plain")]
fn user(uuid: &str) -> Option<&User>
{
    let user = USERS.get(uuid);
    match user {
        Some(u) => Some(u),
        None => None,
    }
}

#[get("/users/<name_grade>?<filters..>")]
fn users(name_grade: NameGrade, filters: Option<Filters>) -> Option<NewUser>
{
    let users: Vec<&User> = USERS
        .values()
        .filter(|user| user.name.contains(&name_grade.name) && user.grade == name_grade.grade)
        .filter(|user|
            if let Some(fts) = &filters {
                user.age == fts.age && user.active == fts.active
            } else {
                true
            }
        )
        .collect();

    if users.len() > 0 {
        Some(NewUser(users))
    } else {
        None
    }
}



#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![user, users])
}

