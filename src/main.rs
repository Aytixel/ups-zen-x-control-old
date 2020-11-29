extern crate hidapi;

use hidapi::HidApi;
use std::{thread, time};

fn main() {
    match HidApi::new() {
        Ok(api) => {
            let device = api.open(1, 0).unwrap();
            loop {
                println!("Ups Info :");
                println!("{}", device.get_indexed_string(3).unwrap().unwrap());
                println!("{}", device.get_indexed_string(29).unwrap().unwrap());
                println!("");
                thread::sleep(time::Duration::new(10, 0));
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

/*

battery :
    higher voltage : 13.8V
    lower voltage : 12V

get_indexed_string(3) ups info :
    input voltage (V)
    ouput voltage needed (V)
    ouput voltage (V)
    leakage current (mA)
    input frequency (Hz)
    battery voltage (V)
    status flag (the first tell if is on battery or not)
get_indexed_string(4) test ups
get_indexed_string(20) switch battery/AC
get_indexed_string(24, 16, 8) ups shutdown
get_indexed_string(29) expected values
    expected minimum input voltage (V)
    expected leakage current (mA)
    expected minimum battery voltage (V)
    expected input frequency (Hz)

*/
