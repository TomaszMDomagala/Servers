use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status::NoContent;
use rocket::response::{content, status};
use rocket::serde::json::Json;

use crate::database;
use crate::models::*;

#[get("/")]
pub fn index() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}

#[get("/servers")]
pub fn all_servers() -> Json<Vec<Server>> {
    use crate::schema::servers::dsl::servers;
    let connection = &mut database::establish_connection();
    servers
        .load::<Server>(connection)
        .map(Json)
        .expect("Error loading servers")
}

#[get("/services")]
pub fn all_services() -> Json<Vec<Service>> {
    use crate::schema::services::dsl::services;
    let connection = &mut database::establish_connection();
    services
        .load::<Service>(connection)
        .map(Json)
        .expect("Error loading servers")
}

#[get("/containers")]
pub fn all_containers() -> Json<Vec<Container>> {
    use crate::schema::containers::dsl::containers;
    let connection = &mut database::establish_connection();
    containers
        .load::<Container>(connection)
        .map(Json)
        .expect("Error loading servers")
}

#[post("/add_server", data = "<server_data>")]
pub fn new_server(server_data: Json<ServerInput>) -> Json<Server> {
    use crate::schema::servers;

    let connection = &mut database::establish_connection();
    diesel::insert_into(servers::table)
        .values(server_data.into_inner())
        .execute(connection)
        .expect("Error adding server");

    Json(
        servers::table
            .order(servers::server_id.desc())
            .first(connection)
            .unwrap(),
    )
}

#[post("/add_service", data = "<service_data>")]
pub fn new_service(service_data: Json<ServiceInput>) -> Json<Service> {
    use crate::schema::services;

    let connection = &mut database::establish_connection();
    diesel::insert_into(services::table)
        .values(service_data.into_inner())
        .execute(connection)
        .expect("Error addind service");

    Json(
        services::table
            .order(services::service_id.desc())
            .first(connection)
            .unwrap(),
    )
}

#[post("/add_container", data = "<container_data>")]
pub fn new_container(container_data: Json<ContainerInput>) -> Json<Container> {
    use crate::schema::containers;

    let connection = &mut database::establish_connection();
    diesel::insert_into(containers::table)
        .values(container_data.into_inner())
        .execute(connection)
        .expect("Error adding service");

    Json(
        containers::table
            .order(containers::container_id.desc())
            .first(connection)
            .unwrap(),
    )
}

#[get("/servers/<s_id>")]
pub fn get_server(s_id: Option<i32>) -> Json<Vec<Server>> {
    let connection = &mut database::establish_connection();
    use crate::schema::servers::dsl::{server_id, servers};

    let query_result: QueryResult<Vec<Server>> = match s_id {
        Some(id) => servers.filter(server_id.eq(id)).load(connection),
        None => servers.load(connection),
    };

    query_result.map(Json).expect("Error loading servers")
}

#[get("/services/<s_id>")]
pub fn get_service(s_id: Option<i32>) -> Json<Vec<Service>> {
    let connection = &mut database::establish_connection();
    use crate::schema::services::dsl::{service_id, services};

    let query_result: QueryResult<Vec<Service>> = match s_id {
        Some(id) => services.filter(service_id.eq(id)).load(connection),
        None => services.load(connection),
    };

    query_result.map(Json).expect("Error loading services")
}

#[get("/containers/<s_id>")]
pub fn get_container(s_id: Option<i32>) -> Json<Vec<Container>> {
    let connection = &mut database::establish_connection();
    use crate::schema::containers::dsl::{container_id, containers};

    let query_result: QueryResult<Vec<Container>> = match s_id {
        Some(id) => containers.filter(container_id.eq(id)).load(connection),
        None => containers.load(connection),
    };

    query_result.map(Json).expect("Error loading containers")
}

#[delete("/servers/<s_id>")]
pub fn delete_server(s_id: i32) -> NoContent {
    use crate::schema::servers::dsl::*;

    let connection = &mut database::establish_connection();
    diesel::delete(servers.filter(server_id.eq(s_id)))
        .execute(connection)
        .expect("Error deleting server");

    NoContent
}

#[delete("/services/<s_id>")]
pub fn delete_service(s_id: i32) -> NoContent {
    use crate::schema::services::dsl::*;

    let connection = &mut database::establish_connection();
    diesel::delete(services.filter(service_id.eq(s_id)))
        .execute(connection)
        .expect("Error deleting service");

    NoContent
}

#[delete("/containers/<s_id>")]
pub fn delete_container(s_id: i32) -> NoContent {
    use crate::schema::containers::dsl::*;

    let connection = &mut database::establish_connection();
    diesel::delete(containers.filter(container_id.eq(s_id)))
        .execute(connection)
        .expect("Error deleting container");

    NoContent
}
