#![windows_subsystem = "windows"]

extern crate fltk;
extern crate hidapi;
extern crate system_shutdown;
extern crate systray;

use fltk::{app::*, button::*, frame::*, image::*, misc::*, window::*};
use hidapi::HidApi;
use std::{ffi::CStr, thread, time};
use system_shutdown::shutdown;
use systray::{Application, Error};

fn window_setup() -> (
    Window,
    Button,
    Button,
    Chart,
    Chart,
    Chart,
    Chart,
    Chart,
    Chart,
    Frame,
) {
    let mut window = Window::new(100, 100, 1000, 360, "Zen-X Control Panel");
    let mut test_button = Button::new(440, 20, 120, 60, "Test");
    let mut shutdown_button = Button::new(440, 140, 120, 60, "Shutdown");
    let mut input_voltage = Chart::new(0, 0, 400, 100, "Input Voltage");
    let mut frequency = Chart::new(0, 120, 400, 100, "Frequency");
    let mut current_intensity = Chart::new(0, 240, 400, 100, "Current Intensity");
    let mut output_voltage = Chart::new(600, 0, 400, 100, "Ouput Voltage");
    let mut battery_voltage = Chart::new(600, 120, 400, 100, "Battery Voltage");
    let mut power_draw = Chart::new(600, 240, 400, 100, "Power Draw");
    let mut battery_state = Frame::new(440, 240, 120, 100, "Battery");
    test_button.set_frame(FrameType::FlatBox);
    test_button.set_color(Color::from_rgb(41, 41, 41));
    test_button.set_label_color(Color::White);
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
    current_intensity.set_type(ChartType::Line);
    current_intensity.set_bounds(0.5, 2.5);
    current_intensity.set_color(Color::from_rgb(41, 41, 41));
    current_intensity.set_label_color(Color::White);
    current_intensity.set_maximum_size(10);
    current_intensity.set_frame(FrameType::FlatBox);
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
    power_draw.set_type(ChartType::Line);
    power_draw.set_bounds(100., 625.);
    power_draw.set_color(Color::from_rgb(41, 41, 41));
    power_draw.set_label_color(Color::White);
    power_draw.set_maximum_size(10);
    power_draw.set_frame(FrameType::FlatBox);
    battery_state.set_label_color(Color::Red);
    window.set_color(Color::from_rgb(51, 51, 51));
    window.set_icon(Some(JpegImage::load("icon.jpg").unwrap()));
    window.end();
    for _ in 0..10 {
        input_voltage.add(0., "", Color::Green);
        frequency.add(0., "", Color::Green);
        current_intensity.add(0., "", Color::Green);
        power_draw.add(0., "", Color::Green);
        output_voltage.add(0., "", Color::Green);
        battery_voltage.add(0., "", Color::Green);
    }
    (
        window,
        test_button,
        shutdown_button,
        input_voltage,
        frequency,
        current_intensity,
        output_voltage,
        battery_voltage,
        power_draw,
        battery_state,
    )
}

fn main() {
    let device_path = CStr::from_bytes_with_nul(
        b"\\\\?\\hid#vid_0001&pid_0000#6&14bd8cd6&0&0000#{4d1e55b2-f16f-11cf-88cb-001111000030}\0",
    )
    .unwrap();
    let app = App::default().with_scheme(AppScheme::Gtk);
    let mut tray = Application::new().unwrap();
    let (
        mut window,
        mut test_button,
        mut shutdown_button,
        mut input_voltage,
        mut frequency,
        mut current_intensity,
        mut output_voltage,
        mut battery_voltage,
        mut power_draw,
        mut battery_state,
    ) = window_setup();
    match HidApi::new() {
        Ok(api) => {
            thread::spawn(move || loop {
                let device = match api.open_path(device_path) {
                    Ok(device) => device,
                    Err(_) => {
                        thread::sleep(time::Duration::new(5, 0));
                        continue;
                    }
                };
                let test_device = match api.open_path(device_path) {
                    Ok(device) => device,
                    Err(_) => {
                        thread::sleep(time::Duration::new(5, 0));
                        continue;
                    }
                };
                let shutdown_device = match api.open_path(device_path) {
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
                let computed_expected_power_draw =
                    (expected_data[1] * expected_data[3]).round() / 10.;
                test_button.set_callback(move || {
                    test_device.get_indexed_string(4).unwrap();
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
                                let computed_power_draw = (data[3] * data[2]).round() / 10.;
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
                                    } else if (data[4] - expected_data[3]).abs() < 5. {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                frequency.set_label(
                                    format!("Frequency : {}Hz / {}Hz", expected_data[3], data[4])
                                        .as_str(),
                                );
                                current_intensity.add(
                                    data[3] / 10.,
                                    "",
                                    if data[3] / 10. <= expected_data[1] / 10. + 1.2 {
                                        Color::Green
                                    } else if data[3] / 10. <= expected_data[1] / 10. + 1.2 {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                current_intensity.set_label(
                                    format!(
                                        "Current Intensity : {}A / {}A",
                                        expected_data[1] / 10.,
                                        data[3] / 10.
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
                                power_draw.add(
                                    computed_power_draw,
                                    "",
                                    if computed_power_draw <= computed_expected_power_draw + 185. {
                                        Color::Green
                                    } else if computed_power_draw
                                        <= computed_expected_power_draw + 385.
                                    {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                power_draw.set_label(
                                    format!(
                                        "Power Draw : {}W / {}W",
                                        computed_expected_power_draw, computed_power_draw
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
    current intensity (dA)
    input frequency (Hz)
    battery voltage (V)
    status flag (the first tell if is on battery or not)
get_indexed_string(4) test ups
get_indexed_string(24, 16, 8) ups shutdown
get_indexed_string(29) expected values
    expected input voltage (V)
    expected current intensity (dA)
    expected battery voltage (V)
    expected input frequency (Hz)

*/
