use std::collections::HashMap;
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Debug, PartialEq)]
pub enum HueState {
    PressLinkButton,
    UsernameCreated { username: String },
}

#[derive(Deserialize, Debug)]
struct HueErrorResponse {
    error: HueError,
}

#[derive(Deserialize, Debug)]
struct HueError {
    r#type: usize,
    address: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct HueUsername {
    username: String,
}

#[derive(Deserialize, Debug)]
struct HueSuccessUsernameResponse {
    success: HueUsername,
}

pub fn ping_hue(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Pinging: {ip} ...");
    reqwest::blocking::get(format!("http://{ip}/api"))?.error_for_status()?;
    Ok(())
}

fn create_resource<T>(hub_ip: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned
{
    let mut map = HashMap::new();
    map.insert("devicetype", "homestuff#local");
    println!("Request: {:?}", map);

    let client = reqwest::blocking::Client::new();

    let res = client.post(format!("http://{}/api/", hub_ip))
        .json(&map)
        .send()?.error_for_status()?;
    let ret = res.json::<T>()?;
    Ok(ret)
}

pub fn create_username(hub_ip: &str) -> Result<HueState, Box<dyn std::error::Error>> {
    let response = create_resource::<Vec<HueErrorResponse>>(hub_ip)?;
    if response[0].error.r#type == 101 {
        Ok(HueState::PressLinkButton)
    } else {
        Err("casdas".into())
    }
}

pub fn create_username_after_pressing(hub_ip: &str) -> Result<HueState, Box<dyn std::error::Error>> {
    let response = create_resource::<Vec<HueSuccessUsernameResponse>>(hub_ip)?;
    let username = &response[0].success.username;
    Ok(HueState::UsernameCreated { username: username.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_username_returns_link_not_pressed() {
        let result = create_username("192.168.1.5");
        assert_eq!(HueState::PressLinkButton, result.unwrap());
    }
}