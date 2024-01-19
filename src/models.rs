use crate::schema::*;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Server {
    pub server_id: i32,
    pub server_name: String,
    pub server_address: String,
    pub server_username: String,
    pub server_password: String,
    pub server_available: Option<bool>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = servers)]
pub struct ServerInput {
    pub server_id: i32,
    pub server_name: String,
    pub server_address: String,
    pub server_username: String,
    pub server_password: String,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Service {
    pub service_id: i32,
    pub service_name: String,
    pub service_port: i32,
    pub service_username: Option<String>,
    pub service_password: Option<String>,
    pub service_available: Option<bool>,
    pub server_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = services)]
pub struct ServiceInput {
    pub service_id: i32,
    pub service_name: String,
    pub service_port: i32,
    pub service_username: Option<String>,
    pub service_password: Option<String>,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Container {
    pub container_id: i32,
    pub container_name: Option<String>,
    pub container_image: Option<String>,
    pub container_state: Option<String>,
    pub container_status: Option<String>,
    pub service_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = containers)]
pub struct ContainerInput {
    pub container_id: i32,
    pub container_name: Option<String>,
    pub container_image: Option<String>,
    pub container_state: Option<String>,
    pub container_status: Option<String>,
}
