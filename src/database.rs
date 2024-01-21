use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn update_server_value_by_id(s_id: i32, new_value: bool) {
    let connection = &mut establish_connection();
    use crate::schema::servers::dsl::*;

    let _result = diesel::update(servers)
        .filter(server_id.eq(s_id))
        .set(server_available.eq(new_value))
        .execute(connection);
}
