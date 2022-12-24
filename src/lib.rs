use std::process::Command;
use std::str;
use regex::Regex;

pub mod philips_hue;

pub fn get_hue_hub_ip() -> Option<String> {
    if cfg!(target_os = "windows") {
        eprintln!("Windows is not yet supported");
        return None;
    }
    let arp_result = command_arp();
    let re = Regex::new(r"\((\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\)").unwrap();
    for cap in re.captures_iter(&arp_result) {
        match  philips_hue::ping_hue(&cap[1]){
            Ok(()) => {
                return Some(String::from(&cap[1]));
            },
            _ => continue
        }
    }
    None
}

fn command_arp() -> String {
    let output = Command::new("arp")
        .arg("-a")
        .output()
        .expect("failed to run `arp -a` command");
    String::from_utf8(output.stdout).expect("couldn't convert arp -a output to string, probably UTF8 encoding issue")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_the_ip() {
        //TODO: How to mock the arp -a command?
        let result = get_hue_hub_ip();

        assert_eq!(Some(String::from("192.168.1.5")), result);
    }
}