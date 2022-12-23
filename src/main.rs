
fn main() {
     let ip = homestuff::get_hue_hub_ip();
     dbg!(&ip);

     homestuff::philips_hue::create_username(&ip.expect("couldn't get it"));
}
