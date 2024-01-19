use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use crate::database;
use crate::models::*;

#[get("/")]
pub fn index() -> Json<Vec<Server>> {
    use crate::schema::servers::dsl::servers;
    let connection = &mut database::establish_connection();
    servers
        .load::<Server>(connection)
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

#[get("/servers?<s_id>")]
pub fn all_servers(s_id: Option<i32>) -> Json<Vec<Server>> {
    let connection = &mut database::establish_connection();
    use crate::schema::servers::dsl::{server_id, servers};

    let query_result: QueryResult<Vec<Server>> = match s_id {
        Some(id) => servers.filter(server_id.eq(id)).load(connection),
        None => servers.load(connection),
    };

    query_result.map(Json).expect("Error loading servers")
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
