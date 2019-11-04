#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello world get -> world"
}

#[get("/<name>")]
fn test(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, world])
        .mount("/test", routes![test])
        .launch();
}
