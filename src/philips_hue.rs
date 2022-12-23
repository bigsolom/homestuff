use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HueErrorResponse {
    error: HueError
}

#[derive(Deserialize, Debug)]
pub struct HueError {
    r#type: usize,
    address: String,
    description: String,
}
#[derive(Deserialize, Debug)]
pub struct HueUsername {
    username: String
}
#[derive(Deserialize, Debug)]
pub struct HueSuccessUsernameResponse {
    success: HueUsername
}

pub fn ping_hue(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Pinging: {ip} ...");
    reqwest::blocking::get(format!("http://{ip}/api/newdeveloper"))?.error_for_status()?;
    Ok(())
}

pub fn create_username(hub_ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("devicetype", "homestuff#local");
    println!("Request: {:?}", map);

    let client = reqwest::blocking::Client::new();

    let res = client.post(format!("http://{}/api/", hub_ip))
        .json(&map)
        .send()?.error_for_status()?;
    println!("Response: {:?}", &res);
    let hue_response: Vec<HueErrorResponse> = res.json()?;
    println!("Response: {:?}", &hue_response);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_username_returns_link_not_pressed() {
        let result = create_username("192.168.1.5");
        match result {
            Ok(h) => assert_eq!(h, ()),
            Err(e) => println!("Error: {}", e)
        }
    }
}