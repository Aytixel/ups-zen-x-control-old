extern crate hidapi;

use fltk::{app::*, button::*, frame::*, window::*};
use hidapi::HidApi;
use std::{thread, time};

fn main() {
    let app = App::default().with_scheme(AppScheme::Gtk);
    let mut wind = Window::new(100, 100, 400, 300, "Zen-X Control Panel");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(0, 200, 400, 100, "Click me!");
    wind.end();
    wind.show();
    //but.set_callback(move || frame.set_label("Hello World!"));
    match HidApi::new() {
        Ok(api) => {
            let device = api.open(1, 0).unwrap();
            let expected_data: Vec<f32> = device.get_indexed_string(29).unwrap().unwrap()[1..21]
                .split(" ")
                .map(|x| x.parse::<f32>().unwrap())
                .collect();
            thread::spawn(move || loop {
                println!("Ups Info :");
                let rawdata = device.get_indexed_string(3).unwrap().unwrap();
                let flags: Vec<u8> = rawdata[38..46]
                    .split("")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();
                let data: Vec<f32> = rawdata[1..32]
                    .split(" ")
                    .map(|x| x.parse::<f32>().unwrap())
                    .collect();
                frame.set_label(&data.iter().fold(String::new(), |acc, a| {
                    acc + &String::from(" ") + &a.to_string()
                }));
                println!("{:?} {:?}", data, flags);
                println!("{:?}", expected_data);
                println!("");
                thread::sleep(time::Duration::new(10, 0));
            });
            app.run().unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
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
