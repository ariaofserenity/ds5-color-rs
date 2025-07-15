extern crate hidapi;

use clap::Parser;
use hidapi::{HidApi, HidDevice};
use std::process::exit;

const VENDOR_ID: u16 = 0x054c;
const EDGE_PRODUCT_ID: u16 = 0x0df2;
const REG_PRODUCT_ID: u16 = 0x0ce6;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[arg(short, long, value_parser = parse_rgb)]
    color: Option<(u8, u8, u8)>,
}

fn main() {
    // Create new hid api
    let api = HidApi::new().expect("Failed to create API");
    let args = Args::parse();
    let devices = api.device_list();
    let mut ds5: Option<HidDevice> = None;

    for device in devices {
        if device.vendor_id() == VENDOR_ID && device.product_id() == REG_PRODUCT_ID {
            println!("Found DualSense controller");
            ds5 = Some(opendevice(&api, VENDOR_ID, REG_PRODUCT_ID));
            break;
            
        } else if device.vendor_id() == VENDOR_ID && device.product_id() == EDGE_PRODUCT_ID {
            println!("Found DualSense Edge Controller");
            ds5 = Some(opendevice(&api, VENDOR_ID, EDGE_PRODUCT_ID));
            break;
        } 
    }

    if ds5.is_none() {
       println!("No compatible device found");
       exit(1);
    }

    let ds5 = ds5.expect("Device not found");
    if let Some(color) = args.color {
        let red = color.0;
        let green = color.1;
        let blue = color.2;
        let report = create_report(red, green, blue);

        println!("Changing LED to colors: R:{} G:{} B:{}", red, green, blue);
        
        ds5.write(&report)
            .expect("Failed to send HID report to controller");
    }
}

fn opendevice(api:&HidApi, vendor_id: u16, product_id: u16) -> HidDevice {
    let ds5 = api.open(vendor_id, product_id);

    match ds5 {
        Ok(r) => r,
        Err(e) => {
            println!("Error while attempting to open device");
            exit(1)
        }

    }
}

fn parse_rgb(s: &str) -> Result<(u8, u8, u8), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err("Expected format 'R,G,B'".to_string());
    }
    let r = parts[0].parse::<u8>().map_err(|_| "Invalid red value")?;
    let g = parts[1].parse::<u8>().map_err(|_| "Invalid green value")?;
    let b = parts[2].parse::<u8>().map_err(|_| "Invalid blue value")?;
    Ok((r, g, b))
}

fn create_report(r: u8, g: u8, b: u8) -> [u8; 48] {
    let mut report = [0u8; 48];
    // rgb
    report[0] = 0x02; // report id
    report[1] = 0xFF;
    report[2] = 0xF7;
    report[45] = r; // red byte
    report[46] = g; // green byte
    report[47] = b; // blue byte

    report
}
