use std::env;
use std::ffi::OsStr;
use std::time::Instant;
use Vec;

#[path = "../input.rs"]
mod input;
#[path = "../pixmap.rs"]
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

    // let mut images = Vec::<image::RgbImage>::new(); // Vec::<image::ImageBuffer<image::Rgb<u8>,Vec<u8>>>::new();
    let images = rayon::join(
        || {
            pixmap::create_pixmap(
                data16,
                channels[0].offset,
                channels[0].scale,
                fft_size,
                step_size,
                width,
            )
        },
        || {
            pixmap::create_pixmap(
                data16,
                channels[1].offset,
                channels[1].scale,
                fft_size,
                step_size,
                width,
            )
        },
    );

    let elapsed = start.elapsed();

    println!(
        "time = {}.{:03}s",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    images.0.save("output_l.png").expect("save l");
    images.1.save("output_r.png").expect("save r");
}
