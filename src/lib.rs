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
        .arg("-a")
        .output()
        .expect("failed to run `arp -a` command");
    let result = str::from_utf8(&output.stdout);
    let re = Regex::new(r"\((\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\)").unwrap();
    for cap in re.captures_iter(&result.expect("couldb't run")) {
        match  ping_hue(&cap[1]){
            Ok(()) => {
                return Some(String::from(&cap[1]));
            },
            _ => continue
        }
    }
    None
}

fn ping_hue(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Pinging: {ip} ...");
    reqwest::blocking::get(format!("http://{ip}/api/newdeveloper"))?.error_for_status()?;
    Ok(())
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