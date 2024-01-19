#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod controller;
mod database;
mod models;
mod schema;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/",
        routes![
            controller::index,
            controller::new_server,
            controller::all_servers,
            controller::delete_server
        ],
    )
}
