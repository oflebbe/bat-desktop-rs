use image::{self};
use rayon::prelude::*;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::{num_complex::Complex32, FftPlanner};
use std::f32::consts::TAU;

fn flo_hue2rgb(p: f32, q: f32, _t: f32) -> f32 {
    let mut t = _t;
    if t < 0.0f32 {
        t += 1.0f32;
    } else if t > 1.0f32 {
        t -= 1.0f32;
    }
    if t < (1.0f32 / 6.0f32) {
        return p + (q - p) * 6.0f32 * t;
    }
    if t < (1.0f32 / 2.0f32) {
        return q;
    }
    if t < (2.0f32 / 3.0f32) {
        return p + (q - p) * ((2.0f32 / 3.0f32) - t) * 6.0f32;
    }
    p
}

fn flo_color888(r: f32, g: f32, b: f32) -> [u8; 3] {
    let r_ = (r * 255.0f32) as u8;
    let g_ = (g * 255.0f32) as u8;
    let b_ = (b * 255.0f32) as u8;

    [r_, g_, b_]
}

fn flo_hsl_to_rgb(h: f32, s: f32, l: f32) -> [u8; 3] {
    let r;
    let g;
    let b;

    if s == 0.0f32 {
        r = l; // achromatic
        g = l;
        b = l;
    } else {
        let q = if l < 0.5f32 {
            l * (1.0f32 + s)
        } else {
            l + s - l * s
        };

        let p = 2.0f32 * l - q;

        r = flo_hue2rgb(p, q, h + 1.0f32 / 3.0f32);
        g = flo_hue2rgb(p, q, h);
        b = flo_hue2rgb(p, q, h - 1.0f32 / 3.0f32);
    }

    flo_color888(r, g, b)
}

pub fn create_pixmap(
    data: &[u16],
    fft_size: usize,
    step_size: usize,
    start: usize,
    width: usize,
) -> (image::RgbImage, image::RgbImage, image::RgbImage) {
    let a0 = 25. / 46. as f32;
    let window: Vec<f32> = (0..fft_size)
        .map(|i| a0 - (1.0 - a0) * (TAU * i as f32 / ((fft_size - 1) as f32)).cos())
        .collect();

    let height = fft_size / 2;

    let mut imgbuf_l = image::RgbImage::new(width as u32 , height as u32);
    let mut imgbuf_r = image::RgbImage::new(width as u32 , height as u32);
    let mut imgbuf_c = image::RgbImage::new(width as u32 , height as u32);
    let max_width = ((data.len() - 1) / 2 - (fft_size - 1) - start) / step_size;
    let w = (start + width).min(max_width);

    let fft = FftPlanner::new().plan_fft_forward(fft_size);
    let ifft = FftPlanner::new().plan_fft_inverse(fft_size);

    let mut pixel_data = Vec::<_>::new();
    (start..w)
        .into_par_iter()
        .map(|col| {
            let mut buffer_l = vec![Complex::zero(); fft_size];
            let mut buffer_r = vec![Complex::zero(); fft_size];
            let mut buffer_c = vec![Complex::zero(); fft_size];
            let mut scratch = vec![Complex::zero(); fft.get_inplace_scratch_len()];

            let i = col * step_size;
            for j in 0..fft_size {
                let val_l = (data[(i + j) * 2 + 0] as f32 - 2048.0f32) * window[j];
                buffer_l[j] = Complex32 { re: val_l, im: 0. };
                let val_r = (data[(i + j) * 2 + 1] as f32 - 2048.0f32) * window[j];
                buffer_r[j] = Complex32 { re: val_r, im: 0. };
            }

            fft.process_with_scratch(&mut buffer_l, &mut scratch);
            fft.process_with_scratch(&mut buffer_r, &mut scratch);

            let mut image_col = vec![([0, 0, 0], [0, 0, 0], [0, 0, 0]); height];

            for j in 0..height {
                let ang = ((buffer_l[j].norm_sqr().log10() - 4.0) / (11.0 - 4.0)).clamp(0.0, 1.0);
                image_col[j].0 = flo_hsl_to_rgb(1.0 - ang, 1.0, 0.5);
            }
            for j in 0..height {
                let ang = ((buffer_r[j].norm_sqr().log10() - 4.0) / (11.0 - 4.0)).clamp(0.0, 1.0);
                image_col[j].1 = flo_hsl_to_rgb(1.0 - ang, 1.0, 0.5);
            }

            for j in 0..fft_size {
                buffer_c[j] = buffer_l[j] * buffer_r[j].conj();
            }

            ifft.process_with_scratch(&mut buffer_c, &mut scratch);

            let max = max_real_complex_f32(&buffer_c);
            let min = min_real_complex_f32(&buffer_c);

            for j in 0..height {
                let ang = (buffer_c[j].re) / (398504800000.0 * 0.005);
                image_col[j].2 = flo_hsl_to_rgb(0.0, 0.0, 1.0 - ang);
            }
            image_col
        })
        .collect_into_vec(&mut pixel_data);
    for (col, pixels) in pixel_data.iter().enumerate() {
        for (i, pixel) in pixels.iter().enumerate() {
            imgbuf_l.put_pixel(
                col as u32,
                (height - 1) as u32 - i as u32,
                image::Rgb(pixel.0),
            );
            imgbuf_r.put_pixel(
                col as u32,
                (height - 1) as u32 - i as u32,
                image::Rgb(pixel.1),
            );
            imgbuf_c.put_pixel(
                col as u32,
                (height - 1) as u32 - i as u32,
                image::Rgb(pixel.2),
            );
        }
    }
    (imgbuf_l, imgbuf_r, imgbuf_c)
}

fn max_real_complex_f32(vec: &Vec<Complex<f32>>) -> f32 {
    vec.iter()
        .fold(None, |min, &x| match min {
            None => Some(x.re),
            Some(y) => Some(x.re.max(y)),
        })
        .unwrap()
}

fn min_real_complex_f32(vec: &Vec<Complex<f32>>) -> f32 {
    vec.iter()
        .fold(None, |min, &x| match min {
            None => Some(x.re),
            Some(y) => Some(x.re.min(y)),
        })
        .unwrap()
}
