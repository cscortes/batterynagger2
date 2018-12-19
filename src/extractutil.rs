use std::fs::File;
use std::io::prelude::*;

fn read_txt_file(fname : &str) -> String {
    let mut f = File::open(fname).expect( "file {} not found.");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.to_lowercase().trim().to_string()
}

pub fn get_bat_info() -> (String, u8, bool) {
    let capinfo = "/sys/class/power_supply/BAT0/capacity";
    let cap = read_txt_file(& capinfo);
    let num = cap.parse::<u8>().unwrap();

    let statusinfo = "/sys/class/power_supply/BAT0/status";
    let status = read_txt_file(& statusinfo);
    let ischarging = status.to_lowercase() == "charging";

    (status, num, ischarging)
}
