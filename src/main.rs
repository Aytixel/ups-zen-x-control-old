#![windows_subsystem = "windows"]

extern crate fltk;
extern crate hidapi;
extern crate system_shutdown;
extern crate systray;

use fltk::{app::*, button::*, frame::*, image::*, misc::*, window::*};
use hidapi::{HidApi, HidDevice};
use std::{
    ffi::CStr,
    thread::{sleep, spawn},
    time::Duration,
};
use systray::{Application, Error};

const EXPECTED_VOLTAGE: f64 = 220.;
const EXPECTED_INTENSITY: f64 = 0.3;
const EXPECTED_BATTERY_VOLTAGE: f64 = 12.;
const EXPECTED_FREQUENCY: f64 = 50.;

fn init_chart(
    mut chart: Chart,
    chart_type: ChartType,
    frame_type: FrameType,
    bounds: (f64, f64),
    color: Color,
    line_color: Color,
    label_color: Color,
    maximum_size: u32,
) -> Chart {
    chart.set_type(chart_type);
    chart.set_frame(frame_type);
    chart.set_bounds(bounds.0, bounds.1);
    chart.set_color(color);
    chart.set_label_color(label_color);
    chart.set_maximum_size(maximum_size);
    for _ in 0..maximum_size {
        chart.add(0., "", line_color);
    }
    chart
}

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
    let mut input_voltage_chart = Chart::new(0, 0, 400, 100, "Input Voltage");
    let mut frequency_chart = Chart::new(0, 120, 400, 100, "Frequency");
    let mut current_intensity_chart = Chart::new(0, 240, 400, 100, "Current Intensity");
    let mut output_voltage_chart = Chart::new(600, 0, 400, 100, "Ouput Voltage");
    let mut battery_voltage_chart = Chart::new(600, 120, 400, 100, "Battery Voltage");
    let mut power_draw_chart = Chart::new(600, 240, 400, 100, "Power Draw");
    let mut battery_state_label = Frame::new(440, 240, 120, 100, "Battery");
    test_button.set_frame(FrameType::FlatBox);
    test_button.set_color(Color::from_rgb(41, 41, 41));
    test_button.set_label_color(Color::White);
    shutdown_button.set_frame(FrameType::FlatBox);
    shutdown_button.set_color(Color::from_rgb(41, 41, 41));
    shutdown_button.set_label_color(Color::White);
    input_voltage_chart = init_chart(
        input_voltage_chart,
        ChartType::Line,
        FrameType::FlatBox,
        (210., 250.),
        Color::from_rgb(41, 41, 41),
        Color::Green,
        Color::White,
        10,
    );
    frequency_chart = init_chart(
        frequency_chart,
        ChartType::Line,
        FrameType::FlatBox,
        (49., 51.),
        Color::from_rgb(41, 41, 41),
        Color::Green,
        Color::White,
        10,
    );
    current_intensity_chart = init_chart(
        current_intensity_chart,
        ChartType::Line,
        FrameType::FlatBox,
        (0.5, 2.5),
        Color::from_rgb(41, 41, 41),
        Color::Green,
        Color::White,
        10,
    );
    output_voltage_chart = init_chart(
        output_voltage_chart,
        ChartType::Line,
        FrameType::FlatBox,
        (210., 250.),
        Color::from_rgb(41, 41, 41),
        Color::Green,
        Color::White,
        10,
    );
    battery_voltage_chart = init_chart(
        battery_voltage_chart,
        ChartType::Line,
        FrameType::FlatBox,
        (11., 14.),
        Color::from_rgb(41, 41, 41),
        Color::Green,
        Color::White,
        10,
    );
    power_draw_chart = init_chart(
        power_draw_chart,
        ChartType::Line,
        FrameType::FlatBox,
        (100., 650.),
        Color::from_rgb(41, 41, 41),
        Color::Green,
        Color::White,
        10,
    );
    battery_state_label.set_label_color(Color::Red);
    window.set_color(Color::from_rgb(51, 51, 51));
    window.set_icon(Some(JpegImage::load("icon.jpg").unwrap()));
    window.end();
    (
        window,
        test_button,
        shutdown_button,
        input_voltage_chart,
        frequency_chart,
        current_intensity_chart,
        output_voltage_chart,
        battery_voltage_chart,
        power_draw_chart,
        battery_state_label,
    )
}

fn shutdown(device: &HidDevice) {
    device.get_indexed_string(24).unwrap();
    system_shutdown::shutdown().unwrap();
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
        mut frequency_chart,
        mut current_intensity_chart,
        mut output_voltage_chart,
        mut battery_voltage_chart,
        mut power_draw_chart,
        mut battery_state_label,
    ) = window_setup();
    match HidApi::new() {
        Ok(api) => {
            spawn(move || loop {
                let device = match api.open_path(device_path) {
                    Ok(device) => device,
                    Err(_) => {
                        sleep(Duration::new(5, 0));
                        continue;
                    }
                };
                let test_device = match api.open_path(device_path) {
                    Ok(device) => device,
                    Err(_) => {
                        sleep(Duration::new(5, 0));
                        continue;
                    }
                };
                let shutdown_device = match api.open_path(device_path) {
                    Ok(device) => device,
                    Err(_) => {
                        sleep(Duration::new(5, 0));
                        continue;
                    }
                };
                let background_device = match api.open_path(device_path) {
                    Ok(device) => device,
                    Err(_) => {
                        sleep(Duration::new(5, 0));
                        continue;
                    }
                };
                let auto_shutdown = spawn(move || loop {
                    match background_device.get_indexed_string(3).unwrap_or(None) {
                        Some(raw_data) => {
                            if raw_data.len() == 47
                                && raw_data[38..39].parse::<u8>().unwrap_or(0) == 1
                            {
                                if raw_data[28..32].parse::<f64>().unwrap_or(0.)
                                    <= EXPECTED_BATTERY_VOLTAGE - 0.1
                                {
                                    shutdown(&background_device);
                                }
                            }
                        }
                        None => {
                            sleep(Duration::new(15, 0));
                            break;
                        }
                    }
                    sleep(Duration::new(20, 0));
                });
                let computed_expected_power_draw = (EXPECTED_INTENSITY * EXPECTED_VOLTAGE).round();
                test_button.set_callback(move || {
                    test_device.get_indexed_string(4).unwrap();
                });
                shutdown_button.set_callback(move || {
                    shutdown(&shutdown_device);
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
                                    if data[0] >= EXPECTED_VOLTAGE + 20. {
                                        Color::Red
                                    } else if data[0] >= EXPECTED_VOLTAGE + 35. {
                                        Color::Yellow
                                    } else if data[0] >= EXPECTED_VOLTAGE {
                                        Color::Green
                                    } else if data[0] >= EXPECTED_VOLTAGE - 3. {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                input_voltage.set_label(
                                    format!("Input Voltage : {}V / {}V", EXPECTED_VOLTAGE, data[0])
                                        .as_str(),
                                );
                                frequency_chart.add(
                                    data[4],
                                    "",
                                    if data[4] == EXPECTED_FREQUENCY {
                                        Color::Green
                                    } else if (data[4] - EXPECTED_FREQUENCY).abs() < 5. {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                frequency_chart.set_label(
                                    format!("Frequency : {}Hz / {}Hz", EXPECTED_FREQUENCY, data[4])
                                        .as_str(),
                                );
                                current_intensity_chart.add(
                                    data[3] / 10.,
                                    "",
                                    if data[3] / 10. <= EXPECTED_INTENSITY + 1.2 {
                                        Color::Green
                                    } else if data[3] / 10. <= EXPECTED_INTENSITY + 1.2 {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                current_intensity_chart.set_label(
                                    format!(
                                        "Current Intensity : {}A / {}A",
                                        EXPECTED_INTENSITY,
                                        data[3] / 10.
                                    )
                                    .as_str(),
                                );
                                output_voltage_chart.add(
                                    data[2],
                                    "",
                                    if data[2] >= EXPECTED_VOLTAGE + 20. {
                                        Color::Red
                                    } else if data[2] >= EXPECTED_VOLTAGE + 35. {
                                        Color::Yellow
                                    } else if data[2] >= EXPECTED_VOLTAGE {
                                        Color::Green
                                    } else if data[2] >= EXPECTED_VOLTAGE - 3. {
                                        Color::Yellow
                                    } else {
                                        Color::Red
                                    },
                                );
                                output_voltage_chart.set_label(
                                    format!(
                                        "Output Voltage : {}V / {}V",
                                        EXPECTED_VOLTAGE, data[2]
                                    )
                                    .as_str(),
                                );
                                battery_voltage_chart.add(
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
                                battery_voltage_chart.set_label(
                                    format!(
                                        "Battery Voltage : {}V / {}V",
                                        data[5], EXPECTED_BATTERY_VOLTAGE
                                    )
                                    .as_str(),
                                );
                                power_draw_chart.add(
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
                                power_draw_chart.set_label(
                                    format!(
                                        "Power Draw : {}W / {}W",
                                        computed_expected_power_draw, computed_power_draw
                                    )
                                    .as_str(),
                                );
                                battery_state_label.set_label_color(if flags[0] == 1 {
                                    Color::Green
                                } else if flags[2] == 1 {
                                    Color::Yellow
                                } else {
                                    Color::Red
                                });
                                battery_state_label.set_label(
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
                        None => {
                            sleep(Duration::new(5, 0));
                            break;
                        }
                    }
                    sleep(Duration::new(1, 0));
                }
                auto_shutdown.join().unwrap();
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
    expected input voltage (220 V)
    expected current intensity (3 dA)
    expected battery voltage (12 V)
    expected input frequency (50 Hz)

*/
