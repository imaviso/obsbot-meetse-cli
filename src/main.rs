#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;
use clap::{Parser, Subcommand, ValueEnum};
use std::ffi::{CStr, CString};

#[derive(Parser)]
#[command(name = "obsbot-cli")]
#[command(about = "CLI for Obsbot Meet SE", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Info {
        #[arg(long)]
        sn: Option<String>,
    },
    Mode {
        #[arg(long)]
        sn: Option<String>,
        #[arg(value_enum)]
        mode: MediaMode,
    },
    Framing {
        #[arg(long)]
        sn: Option<String>,
        #[arg(value_enum)]
        type_: FramingType,
    },
    Hdr {
        #[arg(long)]
        sn: Option<String>,
        #[arg(value_enum)]
        state: Switch,
    },
    /// Image Controls (Brightness, Contrast, etc.)
    Image {
        #[arg(long)]
        sn: Option<String>,
        #[arg(long, help = "Brightness (0-100)")]
        brightness: Option<i32>,
        #[arg(long, help = "Contrast (0-100)")]
        contrast: Option<i32>,
        #[arg(long, help = "Saturation (0-100)")]
        saturation: Option<i32>,
        #[arg(long, help = "Hue (0-100)")]
        hue: Option<i32>,
        #[arg(long, help = "Sharpness (0-100)")]
        sharpness: Option<i32>,
        #[arg(long, help = "Auto White Balance (true/false)")]
        wb_auto: Option<bool>,
        #[arg(
            long,
            help = "White Balance Temperature (2000-8000). Requires --wb-auto false"
        )]
        wb_temp: Option<i32>,
        #[arg(
            long,
            help = "Background Blur Level (0-100). Requires 'Background' mode"
        )]
        blur: Option<i32>,
    },
    Camera {
        #[arg(long)]
        sn: Option<String>,
        #[arg(long, help = "Digital Zoom (1.0-4.0)")]
        zoom: Option<f32>,
        #[arg(long, help = "Auto Focus (true/false)")]
        focus_auto: Option<bool>,
        #[arg(long, help = "Manual Focus (0-100). Requires --focus-auto false")]
        focus: Option<i32>,
        #[arg(long, value_parser = clap::value_parser!(u32).range(0..=3), help = "Anti-Flicker (0=Off, 1=50Hz, 2=60Hz, 3=Auto)")]
        anti_flicker: Option<u32>,
    },
    Reset {
        #[arg(long)]
        sn: Option<String>,
    },
}

#[derive(Clone, ValueEnum)]
enum MediaMode {
    Normal = 0,
    Background = 1,
    AutoFrame = 2,
}

#[derive(Clone, ValueEnum)]
enum FramingType {
    Group = 0,
    Single = 1,
}

#[derive(Clone, ValueEnum)]
enum Switch {
    Off = 0,
    On = 1,
}

use std::thread;
use std::time::Duration;

fn get_device(ctx: ObsbotDevicesCtx, sn: Option<String>) -> Option<ObsbotDeviceCtx> {
    // Retry up to 200 times with 50ms interval (10 seconds total)
    // The SDK device detection can be slow to initialize and add the UVC device.
    for i in 0..200 {
        if i % 20 == 0 && i > 0 {
            println!("Waiting for device... ({}s)", i as f32 * 0.05);
        }

        if let Some(s) = &sn {
            let c_sn = CString::new(s.clone()).unwrap();
            let dev = unsafe { obsbot_devices_get_dev_by_sn(ctx, c_sn.as_ptr()) };
            if !dev.is_null() && unsafe { obsbot_dev_is_inited(dev) } {
                return Some(dev);
            }
        } else {
            // Get first device
            let dev = unsafe { obsbot_devices_get_dev_by_index(ctx, 0) };
            if !dev.is_null() && unsafe { obsbot_dev_is_inited(dev) } {
                return Some(dev);
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    None
}

fn main() {
    let cli = Cli::parse();

    unsafe {
        let ctx = obsbot_devices_get_instance();

        match cli.command {
            Commands::List => {
                let count = obsbot_devices_get_dev_num(ctx);
                println!("Found {} devices", count);
                for i in 0..count {
                    let dev = obsbot_devices_get_dev_by_index(ctx, i);
                    if !dev.is_null() {
                        let mut sn_buf = [0i8; 64];
                        let mut model_buf = [0i8; 64];
                        obsbot_dev_get_sn(dev, sn_buf.as_mut_ptr(), 64);
                        obsbot_dev_get_model(dev, model_buf.as_mut_ptr(), 64);

                        let sn = CStr::from_ptr(sn_buf.as_ptr()).to_string_lossy();
                        let model = CStr::from_ptr(model_buf.as_ptr()).to_string_lossy();
                        println!("Device {}: {} (SN: {})", i, model, sn);
                    }
                }
            }
            Commands::Info { sn } => {
                if let Some(dev) = get_device(ctx, sn) {
                    let mut sn_buf = [0i8; 64];
                    let mut ver_buf = [0i8; 64];
                    let mut model_buf = [0i8; 64];

                    obsbot_dev_get_sn(dev, sn_buf.as_mut_ptr(), 64);
                    obsbot_dev_get_version(dev, ver_buf.as_mut_ptr(), 64);
                    obsbot_dev_get_model(dev, model_buf.as_mut_ptr(), 64);

                    let sn_str = CStr::from_ptr(sn_buf.as_ptr()).to_string_lossy();
                    let ver_str = CStr::from_ptr(ver_buf.as_ptr()).to_string_lossy();
                    let model_str = CStr::from_ptr(model_buf.as_ptr()).to_string_lossy();

                    println!("Model: {}", model_str);
                    println!("Serial: {}", sn_str);
                    println!("Version: {}", ver_str);

                    let media_mode = obsbot_meet_get_media_mode(dev);
                    let hdr = obsbot_meet_get_hdr(dev);

                    println!(
                        "Media Mode: {}",
                        match media_mode {
                            0 => "Normal",
                            1 => "Background",
                            2 => "AutoFrame",
                            _ => "Unknown",
                        }
                    );
                    println!("HDR: {}", if hdr == 1 { "On" } else { "Off" });
                } else {
                    eprintln!("No device found.");
                }
            }
            Commands::Mode { sn, mode } => {
                if let Some(dev) = get_device(ctx, sn) {
                    obsbot_meet_set_media_mode(dev, mode as i32);
                    println!("Set Media Mode.");
                } else {
                    eprintln!("No device found.");
                }
            }
            Commands::Framing { sn, type_ } => {
                if let Some(dev) = get_device(ctx, sn) {
                    obsbot_meet_set_auto_framing_type(dev, type_ as i32);
                    println!("Set Framing Type.");
                } else {
                    eprintln!("No device found.");
                }
            }
            Commands::Hdr { sn, state } => {
                if let Some(dev) = get_device(ctx, sn) {
                    obsbot_meet_set_hdr(dev, state as i32);
                    println!("Set HDR.");
                } else {
                    eprintln!("No device found.");
                }
            }
            Commands::Image {
                sn,
                brightness,
                contrast,
                saturation,
                hue,
                sharpness,
                wb_auto,
                wb_temp,
                blur,
            } => {
                if let Some(dev) = get_device(ctx, sn) {
                    if let Some(val) = brightness {
                        obsbot_image_set_brightness(dev, val);
                        println!("Set Brightness: {}", val);
                    }
                    if let Some(val) = contrast {
                        obsbot_image_set_contrast(dev, val);
                        println!("Set Contrast: {}", val);
                    }
                    if let Some(val) = saturation {
                        obsbot_image_set_saturation(dev, val);
                        println!("Set Saturation: {}", val);
                    }
                    if let Some(val) = hue {
                        obsbot_image_set_hue(dev, val);
                        println!("Set Hue: {}", val);
                    }
                    if let Some(val) = sharpness {
                        obsbot_image_set_sharpness(dev, val);
                        println!("Set Sharpness: {}", val);
                    }
                    if let Some(val) = blur {
                        obsbot_camera_set_background_blur(dev, val);
                        println!("Set Background Blur: {}", val);
                    }

                    if let Some(auto) = wb_auto {
                        let temp = wb_temp.unwrap_or(0);
                        obsbot_image_set_white_balance(dev, if auto { 1 } else { 0 }, temp);
                        println!("Set White Balance: Auto={}, Temp={}", auto, temp);
                    } else if let Some(temp) = wb_temp {
                        // If only temp provided, assume Manual
                        obsbot_image_set_white_balance(dev, 0, temp);
                        println!("Set White Balance: Manual, Temp={}", temp);
                    }
                } else {
                    eprintln!("No device found.");
                }
            }
            Commands::Camera {
                sn,
                zoom,
                focus_auto,
                focus,
                anti_flicker,
            } => {
                if let Some(dev) = get_device(ctx, sn) {
                    if let Some(z) = zoom {
                        obsbot_camera_set_zoom(dev, z);
                        println!("Set Zoom: {}", z);
                    }

                    if let Some(auto) = focus_auto {
                        let val = focus.unwrap_or(0);
                        obsbot_camera_set_focus(dev, if auto { 1 } else { 0 }, val);
                        println!("Set Focus: Auto={}, Val={}", auto, val);
                    } else if let Some(val) = focus {
                        // If only val provided, assume Manual
                        obsbot_camera_set_focus(dev, 0, val);
                        println!("Set Focus: Manual, Val={}", val);
                    }

                    if let Some(val) = anti_flicker {
                        obsbot_camera_set_anti_flicker(dev, val as i32);
                        println!("Set Anti-Flicker: {}", val);
                    }
                } else {
                    eprintln!("No device found.");
                }
            }
            Commands::Reset { sn } => {
                if let Some(dev) = get_device(ctx, sn) {
                    obsbot_camera_reset_default(dev);
                    println!("Reset to default settings.");
                } else {
                    eprintln!("No device found.");
                }
            }
        }
    }
}
