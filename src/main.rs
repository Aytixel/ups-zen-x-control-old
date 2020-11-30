#![windows_subsystem = "windows"]

extern crate fltk;
extern crate hidapi;
extern crate system_shutdown;
extern crate systray;

use fltk::{app::*, button::*, frame::*, image::*, misc::*, window::*};
use hidapi::HidApi;
use std::{thread, time};
use system_shutdown::shutdown;
use systray::{Application, Error};

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
    Frame,
) {
    let mut window = Window::new(100, 100, 1000, 360, "Zen-X Control Panel");
    let mut test_button = Button::new(440, 20, 120, 60, "Test");
    let mut switch_button = Button::new(440, 140, 120, 60, "Switch");
    let mut shutdown_button = Button::new(440, 260, 120, 60, "Shutdown");
    let mut input_voltage = Chart::new(0, 0, 400, 100, "Input Voltage");
    let mut frequency = Chart::new(0, 120, 400, 100, "Frequency");
    let mut output_voltage_needed = Chart::new(600, 0, 400, 100, "Ouput Voltage Needed");
    let mut output_voltage = Chart::new(600, 120, 400, 100, "Ouput Voltage");
    let mut battery_voltage = Chart::new(600, 240, 400, 100, "Battery Voltage");
    let mut battery_state = Frame::new(0, 300, 400, 50, "Battery");
    let mut leakage_current = Frame::new(0, 250, 400, 50, "Leakage Current");
    test_button.set_frame(FrameType::FlatBox);
    test_button.set_color(Color::from_rgb(41, 41, 41));
    test_button.set_label_color(Color::White);
    switch_button.set_frame(FrameType::FlatBox);
    switch_button.set_color(Color::from_rgb(41, 41, 41));
    switch_button.set_label_color(Color::White);
    shutdown_button.set_frame(FrameType::FlatBox);
    shutdown_button.set_color(Color::from_rgb(41, 41, 41));
    shutdown_button.set_label_color(Color::White);
    input_voltage.set_type(ChartType::Line);
    input_voltage.set_bounds(210., 250.);
    input_voltage.set_color(Color::from_rgb(41, 41, 41));
    input_voltage.set_label_color(Color::White);
    input_voltage.set_maximum_size(10);
    input_voltage.set_frame(FrameType::FlatBox);
    frequency.set_type(ChartType::Line);
    frequency.set_bounds(49., 51.);
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
    battery_state.set_label_color(Color::Red);
    leakage_current.set_label_color(Color::White);
    window.set_color(Color::from_rgb(51, 51, 51));
    window.set_icon(Some(JpegImage::load("icon.jpg").unwrap()));
    window.end();
    for _ in 0..10 {
        input_voltage.add(0., "", Color::Green);
        frequency.add(0., "", Color::Green);
        output_voltage_needed.add(0., "", Color::Green);
        output_voltage.add(0., "", Color::Green);
        battery_voltage.add(0., "", Color::Green);
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
        battery_state,
        leakage_current,
    )
}

fn main() {
    let app = App::default().with_scheme(AppScheme::Gtk);
    let mut tray = Application::new().unwrap();
    let (
        mut window,
        mut test_button,
        mut switch_button,
        mut shutdown_button,
        mut input_voltage,
        mut frequency,
        mut output_voltage_needed,
        mut output_voltage,
        mut battery_voltage,
        mut battery_state,
        mut leakage_current,
    ) = window_setup();
    match HidApi::new() {
        Ok(api) => {
            thread::spawn(move || loop {
                let device = match api.open(1, 0) {
                    Ok(device) => device,
                    Err(_) => {
                        thread::sleep(time::Duration::new(5, 0));
                        continue;
                    }
                };
                let test_device = match api.open(1, 0) {
                    Ok(device) => device,
                    Err(_) => {
                        thread::sleep(time::Duration::new(5, 0));
                        continue;
                    }
                };
                let switch_device = match api.open(1, 0) {
                    Ok(device) => device,
                    Err(_) => {
                        thread::sleep(time::Duration::new(5, 0));
                        continue;
                    }
                };
                let shutdown_device = match api.open(1, 0) {
                    Ok(device) => device,
                    Err(_) => {
                        thread::sleep(time::Duration::new(5, 0));
                        continue;
                    }
                };
                let expected_raw_data = device.get_indexed_string(29).unwrap().unwrap();
                let expected_data: Vec<f64> = if expected_raw_data.len() == 22 {
                    expected_raw_data[1..21]
                        .split(" ")
                        .map(|x| x.parse::<f64>().unwrap_or(0.))
                        .collect()
                } else {
                    continue;
                };
                test_button.set_callback(move || {
                    test_device.get_indexed_string(4).unwrap();
                });
                switch_button.set_callback(move || {
                    let is_on_battery = switch_device.get_indexed_string(3).unwrap().unwrap()
                        [43..44]
                        .parse::<u8>()
                        .unwrap();
                    if is_on_battery == 1 {
                        switch_device.get_indexed_string(4).unwrap();
                    } else {
                        switch_device.get_indexed_string(5).unwrap();
                    }
                });
                shutdown_button.set_callback(move || {
                    shutdown_device.get_indexed_string(24).unwrap();
                    shutdown().unwrap();
                });
                loop {
                    match device.get_indexed_string(3).unwrap_or(None) {
                        Some(raw_data) => {
                            if raw_data.len() == 47 {
                                let flags: Vec<u8> = raw_data[38..46]
                                    .split("")
                                    .filter(|x| x.len() > 0)
                                    .map(|x| x.parse::<u8>().unwrap_or(0))
                                    .collect();
                                let data: Vec<f64> = raw_data[1..32]
                                    .split(" ")
                                    .map(|x| x.parse::<f64>().unwrap_or(0.))
                                    .collect();
                                input_voltage.add(
                                    data[0],
                                    "",
                                    if data[0] >= expected_data[0] + 20. {
                                        Color::Red
                                    } else if data[0] >= expected_data[0] + 35. {
                                        Color::Yellow
                                    } else if data[0] >= expected_data[0] {
                                        Color::Green
                                    } else if data[0] >= expected_data[0] - 3. {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                input_voltage.set_label(
                                    format!("Input Voltage : {}V / {}V", expected_data[0], data[0])
                                        .as_str(),
                                );
                                frequency.add(
                                    data[4],
                                    "",
                                    if data[4] == expected_data[3] {
                                        Color::Green
                                    } else if (data[4] - expected_data[3]).abs() < 0.3 {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                frequency.set_label(
                                    format!("Frequency : {}Hz / {}Hz", expected_data[3], data[4])
                                        .as_str(),
                                );
                                output_voltage_needed.add(
                                    data[1],
                                    "",
                                    if data[1] >= expected_data[0] + 20. {
                                        Color::Red
                                    } else if data[1] >= expected_data[0] + 35. {
                                        Color::Yellow
                                    } else if data[1] >= expected_data[0] {
                                        Color::Green
                                    } else if data[1] >= expected_data[0] - 3. {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                output_voltage_needed.set_label(
                                    format!(
                                        "Output Voltage Needed : {}V / {}V",
                                        expected_data[0], data[1]
                                    )
                                    .as_str(),
                                );
                                output_voltage.add(
                                    data[2],
                                    "",
                                    if data[2] >= expected_data[0] + 20. {
                                        Color::Red
                                    } else if data[2] >= expected_data[0] + 35. {
                                        Color::Yellow
                                    } else if data[2] >= expected_data[0] {
                                        Color::Green
                                    } else if data[2] >= expected_data[0] - 3. {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                output_voltage.set_label(
                                    format!(
                                        "Output Voltage : {}V / {}V",
                                        expected_data[0], data[2]
                                    )
                                    .as_str(),
                                );
                                battery_voltage.add(
                                    data[5],
                                    "",
                                    if data[5] >= 12.72 {
                                        Color::Green
                                    } else if data[5] >= 12. {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                battery_voltage.set_label(
                                    format!(
                                        "Battery Voltage : {}V / {}V",
                                        data[5], expected_data[2]
                                    )
                                    .as_str(),
                                );
                                battery_state.set_label_color(if flags[0] == 1 {
                                    Color::Green
                                } else if flags[2] == 1 {
                                    Color::Yellow
                                } else {
                                    Color::Red
                                });
                                battery_state.set_label(
                                    format!(
                                        "Battery : {}",
                                        if flags[0] == 1 || flags[2] == 1 {
                                            "On"
                                        } else {
                                            "Off"
                                        }
                                    )
                                    .as_str(),
                                );
                                leakage_current.set_label(
                                    format!(
                                        "Leakage Current : {}mA / {}mA",
                                        expected_data[1], data[3]
                                    )
                                    .as_str(),
                                );
                                app.redraw();
                            }
                        }
                        None => break,
                    }
                    thread::sleep(time::Duration::new(1, 0));
                }
                thread::sleep(time::Duration::new(5, 0));
            });
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    tray.set_icon_from_file("icon.ico").unwrap();
    tray.set_tooltip("Zen-X Control Panel").unwrap();
    tray.add_menu_item("Show", move |_| {
        window.show();
        app.run().unwrap();
        Ok::<_, Error>(())
    })
    .unwrap();
    tray.wait_for_message().unwrap();
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
get_indexed_string(5) switch to battery
get_indexed_string(24, 16, 8) ups shutdown
get_indexed_string(29) expected values
    expected minimum input voltage (V)
    expected leakage current (mA)
    expected minimum battery voltage (V)
    expected input frequency (Hz)

*/
