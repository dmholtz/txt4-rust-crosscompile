use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;

#[derive(Debug, Copy, Clone)]
struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl RgbColor {
    const BLACK: RgbColor = RgbColor { red: 0, green: 0, blue: 0 };
    const RED: RgbColor = RgbColor { red: 255, green: 0, blue: 0 };
    const GREEN: RgbColor = RgbColor { red: 0, green: 255, blue: 0 };
    const BLUE: RgbColor = RgbColor { red: 0, green: 0, blue: 255 };
    const PURPLE: RgbColor = RgbColor { red: 255, green: 0, blue: 255 };
    const YELLOW: RgbColor = RgbColor { red: 255, green: 255, blue: 0 };
    const CYAN: RgbColor = RgbColor { red: 0, green: 255, blue: 255 };
    const WHITE: RgbColor = RgbColor { red: 255, green: 255, blue: 255 };
}

/// Set the TXT-4.0 RGB LED color.
/// 
/// The multi-color LED on the TXT-4.0 can be controlled by writing values (0-255) to files in
/// the /sys/class/leds/ directory. This function sets the red, green, and blue components of the LED.
///
/// # Arguments
/// * `color` - RgbColor struct with red, green, blue values (0-255)
fn set_txt_led_color(color: RgbColor) -> Result<(), std::io::Error> {
    let make_path = |name: &str| format!("/sys/class/leds/{name}/brightness");

    let mut red_file = File::create(make_path("LED_RED"))?;
    let mut green_file = File::create(make_path("LED_GREEN"))?;
    let mut blue_file = File::create(make_path("LED_BLUE"))?;
    
    // write! macro avoids allocation of intermediate Strings
    write!(red_file,   "{}", color.red)?;
    write!(green_file, "{}", color.green)?;
    write!(blue_file,  "{}", color.blue)?;
    Ok(())
}

#[cfg(target_arch = "arm")]
fn main() {
    println!("\nRGB LED example\n");

    let color_sequence = [
        RgbColor::RED,
        RgbColor::GREEN,
        RgbColor::BLUE,
        RgbColor::PURPLE,
        RgbColor::YELLOW,
        RgbColor::CYAN,
        RgbColor::WHITE,
        RgbColor::BLACK,
    ];
    for rgb_color in color_sequence {
        _ = set_txt_led_color(rgb_color).unwrap();
        sleep(Duration::from_millis(500));
    }
}