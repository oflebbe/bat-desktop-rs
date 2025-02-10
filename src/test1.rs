use std::env;
use std::ffi::OsStr;
use std::time::Instant;
use Vec;

mod input;
mod pixmap;

use rayon;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename = OsStr::new(&args[1]);

    let input = input::Input::new(filename).expect("no err expected");

    let data16 = input.get();
    let channels = &input.channels;

    let fft_size = 512;
    let step_size = (fft_size as f32 / 0.9f32) as usize;

    let width = ((data16.len() - channels[0].offset) / channels[0].scale - fft_size) / step_size;

    let images = 
            pixmap::create_pixmap(
                data16,
                fft_size,
                step_size,
                0,
                width);
    
    let elapsed = start.elapsed();

    println!(
        "time = {}.{:03}s",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    images.0.save("output_l.png").expect("save l");
    images.1.save("output_r.png").expect("save r");
}
