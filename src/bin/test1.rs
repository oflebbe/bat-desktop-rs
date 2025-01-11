use std::env;

use std::ffi::OsStr;
use std::time::Instant;

use Vec;
// Perform a forward FFT of size 1234
// use eframe::egui;

#[path="../pixmap.rs"]
mod pixmap;
#[path="../input.rs"]
mod input;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename = OsStr::new(&args[1]);

    let input = input::Input::new(filename).expect("no err expected");

    let data16 = input.get();
    let channels = &input.channels;

    /* let mut sum = 0;
    for i in 0 .. data16.len() {
        sum += data16[i] as u64;
    }
    println!("{} size, {} sum", data16.len(), sum); */

    let img_l = pixmap::create_pixmap(
        data16,
        channels[0].offset+2000000,
        channels[0].scale,
        2000,
        512,
        0.96,
    );

    let img_r = pixmap::create_pixmap(
        data16,
        channels[1].offset+2000000,
        channels[1].scale,
        2000,
        512,
        0.96,
    );
    let elapsed = start.elapsed();
    println!(
        "time = {}.{:03}",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
    img_l.save("output_l.png").expect("save l");
    img_r.save("output_r.png").expect("save r");
    
}