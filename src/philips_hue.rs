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

pub struct PhilipsHueClient {
    pub ip: String
}

impl PhilipsHueClient {
    pub fn ping(&self) -> Result<(), Box<dyn std::error::Error>> {
        reqwest::blocking::get(format!("{}/api", self.ip))?.error_for_status()?;
        Ok(())
    }

    fn create_resource<T>(&self) -> Result<T, Box<dyn std::error::Error>>
        where
            T: DeserializeOwned
    {
        let mut map = HashMap::new();
        map.insert("devicetype", "homestuff#local");
        println!("Request: {:?}", map);

        let client = reqwest::blocking::Client::new();

        let res = client.post(format!("{}/api", self.ip))
            .json(&map)
            .send()?.error_for_status()?;
        let ret = res.json::<T>()?;
        Ok(ret)
    }

    pub fn register_application_on_hue(&self) -> Result<HueState, Box<dyn std::error::Error>> {
        let response = self.create_resource::<Vec<HueErrorResponse>>()?;
        if response[0].error.r#type == 101 {
            Ok(HueState::PressLinkButton)
        } else {
            Err("Should've gotten Press Link Button error".into())
        }
    }

    pub fn create_username(&self) -> Result<HueState, Box<dyn std::error::Error>> {
        let response = self.create_resource::<Vec<HueSuccessUsernameResponse>>()?;
        let username = &response[0].success.username;
        Ok(HueState::UsernameCreated { username: username.clone() })
    }
}

#[cfg(test)]
use mockito;
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    // https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
    // fn run_test<T>(test: T) -> ()
    //     where T: FnOnce() -> () + panic::UnwindSafe
    // {
    //     setup();
    //     let result = panic::catch_unwind(|| {
    //         test()
    //     });
    //
    //     teardown();
    //
    //     assert!(result.is_ok())
    // }
    //
    // fn setup() {
    //
    //
    // }

    #[test]
    fn hue_client_ping_succeeds_for_hue_ip() {
        let ip = &mockito::server_url();
        let client = PhilipsHueClient { ip: String::from(ip) };
        let _m = mock("GET", "/api")
            .with_status(201)
            .create();
        assert_eq!(client.ping().unwrap(), ());
    }

    #[test]
    #[should_panic]
    fn hue_client_ping_fails_for_non_hue_ip() {
        let ip = &mockito::server_url();
        let client = PhilipsHueClient { ip: String::from(ip) };
        let _m = mock("GET", "/api")
            .with_status(404)
            .create();
        client.ping().unwrap();
    }


    #[test]
    fn register_app_on_hue_returns_link_not_pressed() {
        let ip = &mockito::server_url();
        let client = PhilipsHueClient { ip: String::from(ip) };
        let _m = mock("POST", "/api")
            .with_status(200)
            .with_body(r#"[
                {
                    "error": {
                        "type": 101,
                        "address": "",
                        "description": "link button not pressed"
                    }
                }
            ]"#)
            .create();
        let result = client.register_application_on_hue();
        assert_eq!(HueState::PressLinkButton, result.unwrap());
    }

    #[test]
    fn create_username_works() {
        let ip = &mockito::server_url();
        let client = PhilipsHueClient { ip: String::from(ip) };
        let _m = mock("POST", "/api")
            .with_status(200)
            .with_body(r#"[
                {
                    "success": {
                        "username": "abc123"
                    }
                }
            ]"#)
            .create();
        let result = client.create_username();
        assert_eq!(HueState::UsernameCreated {username: String::from("abc123")}, result.unwrap());
    }
}