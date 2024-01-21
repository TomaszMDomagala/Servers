use chrono::Utc;
use cron::Schedule;
use diesel::prelude::*;
use std::str::FromStr;
use tokio::time::Duration;

use crate::database;
use crate::models::*;
use crate::utils;

pub async fn schedule_crono_task(cron_expr: String) {
    let schedule = Schedule::from_str(&cron_expr).unwrap();

    loop {
        tasks().await;

        let next_time = schedule.upcoming(Utc).next().unwrap();
        let duration = next_time.signed_duration_since(Utc::now());
        tokio::time::sleep(Duration::from_secs(
            duration.num_seconds().try_into().unwrap(),
        ))
        .await;
    }
}

async fn tasks() {
    check_avability().await;
}

async fn check_avability() {
    use crate::schema::servers::dsl::servers;
    let connection = &mut database::establish_connection();

    let items = servers.load::<Server>(connection).unwrap();
    for item in items {
        let result = utils::ping(item.server_address)
            .await
            .expect("Error Pinging");
        database::update_server_value_by_id(item.server_id, result);
    }
}
