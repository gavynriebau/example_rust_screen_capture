
extern crate scrap;
extern crate image;

use scrap::*;
use std::thread;
use std::time::Duration;
use std::path::Path;

fn main() {

    let display = Display::main().unwrap();
    let mut capturer = Capturer::new(display).unwrap();
    let width = capturer.width() as u32;
    let height = capturer.height() as u32;

    // Sleep for 1s to allow time for capturer to gather at least 1 frame.
    thread::sleep(Duration::from_millis(1000));

    let frame = capturer.frame().unwrap();

    println!("Captured frame");

    image::save_buffer(&Path::new("test.png"), &frame, width, height, image::RGBA(8)).unwrap();
}
