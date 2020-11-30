extern crate hidapi;

use fltk::{app::*, button::*, frame::*, misc::*, window::*};
use hidapi::HidApi;
use std::{thread, time};

fn window_setup() -> (
    Window,
    Button,
    Button,
    Button,
    Chart,
    Chart,
    Chart,
    Chart,
    Chart,
    Frame,
) {
    let mut window = Window::new(100, 100, 1280, 720, "Zen-X Control Panel");
    let frame = Frame::new(0, 300, 400, 200, "");
    let test_button = Button::new(0, 500, 400, 100, "Test");
    let switch_button = Button::new(0, 500, 400, 100, "Switch");
    let shutdown_button = Button::new(0, 500, 400, 100, "Shutdown");
    let mut input_voltage = Chart::new(0, 0, 400, 100, "Input Voltage");
    let mut frequency = Chart::new(0, 120, 400, 100, "Frequency");
    let mut output_voltage_needed = Chart::new(600, 0, 400, 100, "Ouput Voltage Needed");
    let mut output_voltage = Chart::new(600, 120, 400, 100, "Ouput Voltage");
    let mut battery_voltage = Chart::new(600, 240, 400, 100, "Battery Voltage");
    input_voltage.set_type(ChartType::Line);
    input_voltage.set_bounds(210., 250.);
    input_voltage.set_color(Color::from_rgb(41, 41, 41));
    input_voltage.set_label_color(Color::White);
    input_voltage.set_maximum_size(10);
    input_voltage.set_frame(FrameType::FlatBox);
    frequency.set_type(ChartType::Line);
    frequency.set_bounds(47.5, 52.5);
    frequency.set_color(Color::from_rgb(41, 41, 41));
    frequency.set_label_color(Color::White);
    frequency.set_maximum_size(10);
    frequency.set_frame(FrameType::FlatBox);
    output_voltage_needed.set_type(ChartType::Line);
    output_voltage_needed.set_bounds(210., 250.);
    output_voltage_needed.set_color(Color::from_rgb(41, 41, 41));
    output_voltage_needed.set_label_color(Color::White);
    output_voltage_needed.set_maximum_size(10);
    output_voltage_needed.set_frame(FrameType::FlatBox);
    output_voltage.set_type(ChartType::Line);
    output_voltage.set_bounds(210., 250.);
    output_voltage.set_color(Color::from_rgb(41, 41, 41));
    output_voltage.set_label_color(Color::White);
    output_voltage.set_maximum_size(10);
    output_voltage.set_frame(FrameType::FlatBox);
    battery_voltage.set_type(ChartType::Line);
    battery_voltage.set_bounds(11., 14.);
    battery_voltage.set_color(Color::from_rgb(41, 41, 41));
    battery_voltage.set_label_color(Color::White);
    battery_voltage.set_maximum_size(10);
    battery_voltage.set_frame(FrameType::FlatBox);
    window.set_color(Color::from_rgb(51, 51, 51));
    window.end();
    window.show();
    for _ in 0..10 {
        input_voltage.add(0., "", Color::White);
        frequency.add(0., "", Color::White);
        output_voltage_needed.add(0., "", Color::White);
        output_voltage.add(0., "", Color::White);
        battery_voltage.add(0., "", Color::White);
    }
    (
        window,
        test_button,
        switch_button,
        shutdown_button,
        input_voltage,
        frequency,
        output_voltage_needed,
        output_voltage,
        battery_voltage,
        frame,
    )
}

fn main() {
    let app = App::default().with_scheme(AppScheme::Gtk);
    let (
        window,
        test_button,
        switch_button,
        shutdown_button,
        mut input_voltage,
        mut frequency,
        mut output_voltage_needed,
        mut output_voltage,
        mut battery_voltage,
        mut frame,
    ) = window_setup();
    //but.set_callback(move || frame.set_label("Hello World!"));
    match HidApi::new() {
        Ok(api) => {
            let device = api.open(1, 0).unwrap();
            let expected_data: Vec<f64> = device.get_indexed_string(29).unwrap().unwrap()[1..21]
                .split(" ")
                .map(|x| x.parse::<f64>().unwrap())
                .collect();
            thread::spawn(move || loop {
                println!("Ups Info :");
                let rawdata = device.get_indexed_string(3).unwrap().unwrap();
                let flags: Vec<u8> = rawdata[38..46]
                    .split("")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();
                let data: Vec<f64> = rawdata[1..32]
                    .split(" ")
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect();
                println!("{:?} {:?}", data, flags);
                input_voltage.add(data[0], "", Color::White);
                input_voltage.set_label(format!("Input Voltage : {}", data[0]).as_str());
                frequency.add(data[4], "", Color::White);
                frequency.set_label(format!("Frequency : {}", data[4]).as_str());
                output_voltage_needed.add(data[1], "", Color::White);
                output_voltage_needed
                    .set_label(format!("Output Voltage Needed : {}", data[1]).as_str());
                output_voltage.add(data[2], "", Color::White);
                output_voltage.set_label(format!("Output Voltage : {}", data[2]).as_str());
                battery_voltage.add(data[5], "", Color::White);
                battery_voltage.set_label(format!("Battery Voltage : {}", data[5]).as_str());
                println!("{:?}", expected_data);
                println!("");
                thread::sleep(time::Duration::new(1, 0));
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
