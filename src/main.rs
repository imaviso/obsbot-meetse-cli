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
    /// List connected devices
    List,
    /// Get device information
    Info {
        /// Serial number of the device (optional, uses first found if not provided)
        #[arg(long)]
        sn: Option<String>,
    },
    /// Control Media Mode (Background, AutoFrame, Normal)
    Mode {
        #[arg(long)]
        sn: Option<String>,
        #[arg(value_enum)]
        mode: MediaMode,
    },
    /// Control Auto Framing (Group, Single)
    Framing {
        #[arg(long)]
        sn: Option<String>,
        #[arg(value_enum)]
        type_: FramingType,
    },
    /// Control HDR
    Hdr {
        #[arg(long)]
        sn: Option<String>,
        #[arg(value_enum)]
        state: Switch,
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

fn get_device(ctx: ObsbotDevicesCtx, sn: Option<String>) -> Option<ObsbotDeviceCtx> {
    unsafe {
        if let Some(s) = sn {
            let c_sn = CString::new(s).ok()?;
            let dev = obsbot_devices_get_dev_by_sn(ctx, c_sn.as_ptr());
            if dev.is_null() {
                None
            } else {
                Some(dev)
            }
        } else {
            // Get first device
            let count = obsbot_devices_get_dev_num(ctx);
            if count > 0 {
                let dev = obsbot_devices_get_dev_by_index(ctx, 0);
                if dev.is_null() {
                    None
                } else {
                    Some(dev)
                }
            } else {
                None
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    unsafe {
        let ctx = obsbot_devices_get_instance();

        // Wait a bit for discovery? The SDK might be async.
        // For a CLI, we assume devices are enumerated quickly or we might need a small sleep.
        // Let's try listing immediately.

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
        }
    }
}
