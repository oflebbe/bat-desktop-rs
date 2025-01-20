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

    let fft_size = 512;
    let step_size = (fft_size as f32 / 0.9f32) as usize;

    let width = (((data16.len()-channels[0].offset)/channels[0].scale-fft_size))/step_size;

    
    let img_l = pixmap::create_pixmap(
        data16,
        channels[0].offset,
        channels[0].scale,
        fft_size, step_size, width);


    let img_r = pixmap::create_pixmap(
        data16,
        channels[1].offset,
        channels[1].scale,
        fft_size, step_size, width);
    
    let elapsed = start.elapsed();
    
    println!(
        "time = {}.{:05}",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );


    img_l.save("output_l.png").expect("save l");
    img_r.save("output_r.png").expect("save r");
    
}