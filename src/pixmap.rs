use colorsys::{Hsl, HslRatio, Rgb};
use image::{self, ImageBuffer, RgbImage};
use std::f32::consts::TAU;


#[cfg(feature = "meow")]
#[path="pixmap/fft_meow.rs"]
mod fft;

#[cfg(not(feature = "meow"))]
#[path="pixmap/fft_rustfft.rs"]
mod fft;


use rayon::prelude::*;

pub fn create_pixmap(
    data: &[u16],
    offset: usize,
    scale: usize,
    width: usize,
    fft_size: usize,
    overlap: f32,
) -> image::RgbImage {
    let mut window = vec![0.0f32; fft_size];
    let a0 = 25. / 46. as f32;
    for i in 0..fft_size {
        window[i] = a0 - (1.0 - a0) * (TAU * i as f32 / ((fft_size - 1) as f32)).cos();
    }

    let off = (fft_size as f32 * (1.0f32 - overlap)) as usize;
    let mut end = width * off + offset;
    let mut fft_end = end + fft_size;
    if fft_end > data.len() {
        fft_end = data.len();
        end = fft_end - fft_size;
    }

    let size = (data.len() / 2 - fft_size) / off;



    let mut fft = fft::Fft::new(fft_size);
    let height = fft_size / 2;

    let mut imgbuf = image::RgbImage::new(width as u32, height as u32);

    for col in 0..width {
        let i = col * off + offset;
        for j in 0..fft_size {
            let val = (data[(i + j) * scale] as f32 - 2048.0f32) * window[j];
            fft.input_set_real(j, val);
        }

        fft.process();
       

        for j in 0..height {
            let power = fft.output_power(j);
            
            let mut ang = (power.log10() - 4.0f32) / (11.0f32 - 4.0f32);
            if ang < 0.0 {
                ang = 0.0;
            }
            if ang > 1.0 {
                ang = 1.0;
            }
            let v: [f32; 3] = [1.0 - ang as f32, 1.0f32, 0.5f32];

            let hsl: Hsl = HslRatio::from(&v).into();
            let rgb = Rgb::from(hsl);

            imgbuf.put_pixel(
                col as u32,
                (height - 1 - j) as u32,
                image::Rgb([rgb.red() as u8, rgb.green() as u8, rgb.blue() as u8]),
            );
        }
    }

    imgbuf
}
