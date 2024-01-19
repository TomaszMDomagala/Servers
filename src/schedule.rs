use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use tokio::task;

fn main() {
    schedule_task();
}

pub fn schedule_task() {
    let expression = "*/30 * * * *";
    let schedule = Schedule::from_str(expression).unwrap();
    println!("Upcoming fire times: ");
    for datetime in schedule.upcoming(Utc).take(10) {
        println!("-> {}", datetime);
    }
    let join = task::spawn_blocking
}
