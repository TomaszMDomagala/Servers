#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use std::env;

// use rocket::{Build, Rocket};

mod controller;
mod database;
mod models;
mod schema;
mod tasks;
mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let r = rocket::build()
        .mount(
            "/",
            routes![
                controller::index,
                controller::new_server,
                controller::new_service,
                controller::new_container,
                controller::get_server,
                controller::get_service,
                controller::get_container,
                controller::all_servers,
                controller::all_services,
                controller::all_containers,
                controller::delete_server,
                controller::delete_service,
                controller::delete_container
            ],
        )
        .ignite()
        .await?;
    tokio::spawn(async move {
        tasks::schedule_crono_task(env::var("PING_CRON").expect("PING_CRON must be set")).await
    });
    r.launch().await?;

    Ok(())
}
