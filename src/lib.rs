use std::process::Command;
use std::str;
use std::collections::HashMap;
use std::io::Error;

use regex::Regex;

pub fn get_hue_hub_ip() -> Option<String> {
    if cfg!(target_os = "windows") {
        eprintln!("Windows is not yet supported");
        return None;
    }
    let output = Command::new("arp")
        // .arg("-c")
        // .arg("arp")
        .arg("-a")
        .output()
        .expect("failed to run `arp -a` command");
    let result = str::from_utf8(&output.stdout);
    let re = Regex::new(r"\((\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\)").unwrap();
    let mut ip = "";
    for cap in re.captures_iter(&result.expect("couldb't run")) {
        match  ping_hue(&cap[1]){
            Ok(true) => {
                return Some(String::from(&cap[1].to_string()));
            },
            _ => continue
        }
    }
    None
}

fn ping_hue(ip: &str) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Pinging: {ip} ...");
    let resp = reqwest::blocking::get(format!("http://{ip}/api/newdeveloper"))?;
    Ok(resp.status().is_success())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_the_ip() {
        //TODO: How to mock the arp -a command?
        let result = get_hue_hub_ip();

        assert_eq!(Some(String::from("eslam")), result);
    }
}