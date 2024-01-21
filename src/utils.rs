use regex::Regex;
use std::process::Command;

pub async fn ping(ip_address: String) -> Result<bool, &'static str> {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip_address)
        .output()
        .map_err(|_| "Failed to execute ping")?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    let regex = Regex::new("[1-9].packets.transmitted.+[1-9].packets.received").unwrap();
    if regex.is_match(&output_str) {
        Ok(true)
    } else {
        Err("Error matching regex")
    }
}
