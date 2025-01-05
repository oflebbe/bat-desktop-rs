use colorsys::{Hsl, HslRatio, Rgb, RgbRatio};
use image::{self, ImageBuffer};
use std::env;
use std::error::Error;
use std::f32::consts::TAU;
use std::ffi::OsStr;

use Vec;
// Perform a forward FFT of size 1234
// use eframe::egui;
/*
use rustfft::{
    num_complex::{Complex, Complex32},
    FftPlanner,
}; */

#[cfg(feature = "mmap")]
struct Input {
    buf: memmap::Mmap,
}

#[cfg(not(feature = "mmap"))]
struct Input {
    buf: Vec<u8>,
}

#[cfg(feature = "mmap")]
impl Input {
    fn new(filename: &OsStr) -> Result<Input, Box<dyn Error>> {
        let file = File::open(filename)?;
        return Ok(Input {
            buf: unsafe { memmap::Mmap::map(&file)? },
        });
    }
}

#[cfg(not(feature = "mmap"))]
impl Input {
    fn new(filename: &OsStr) -> Result<Input, Box<dyn Error>> {
        return Ok(Input {
            buf: std::fs::read(filename)?,
        });
    }
}

impl Input {
    fn get(&self) -> &[u16] {
        let (_, data16, _) = unsafe { self.buf.align_to::<u16>() };
        data16
    }
}

fn create_pixmap(
    data: &[u16],
    start: usize,
    width: usize,
    fft_size: usize,
    overlap: f32,
) -> image::RgbImage {
    let mut window = vec![0.0f32; fft_size];
    let a0 = 25. / 46. as f32;
    for i in 0..fft_size {
        window[i] = a0 - (1.0 - a0) * (TAU * i as f32 / (fft_size - 1) as f32).cos();
    }

    let off = (fft_size as f32 * (1.0f32 - overlap)) as usize;
    let mut end = width * off + start;
    let mut fft_end = end + fft_size;
    if fft_end > data.len() {
        fft_end = data.len();
        end = fft_end - fft_size;
    }

    let size = (data.len() / 2 - fft_size) / off;
    /*
    let mut planner: FftPlanner<f32> = FftPlanner::new();
    let fft = planner.plan_fft_forward(FFT_SIZE);
    let mut buffer = [Complex32 { re: 0.0, im: 0.0 }; FFT_SIZE];
    */
    let mut workset = bat_desktop_rs::fft_generate_workset_real(fft_size as i32);
    let mut fft_out = vec![bat_desktop_rs::FftComplex { r: 0.0, j: 0.0 }; fft_size];
    let mut fft_in = vec![0.0 as libc::c_float; fft_size];
    let height = fft_size / 2;

    let mut imgbuf = image::RgbImage::new(size as u32, height as u32);

    for col in 0..width {
        let i = col * off + start;
        for j in 0..fft_size {
            fft_in[j] = (data[i + j] as f32 - 2048.0f32) * window[j];
        }

        bat_desktop_rs::fft_real(&mut workset, &mut fft_in, &mut fft_out);

        for j in 0..height {
            let power = fft_out[j].r * fft_out[j].r + fft_out[j].j * fft_out[j].j;
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

            let pixel = imgbuf.get_pixel_mut(i as u32, (height - j - 1) as u32);
            *pixel = image::Rgb([
                (rgb.red() * 255.0) as u8,
                (rgb.green() * 255.0) as u8,
                (rgb.blue() * 255.0) as u8,
            ]);
        }
    }
    imgbuf
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = OsStr::new(&args[1]);

    let input = Input::new(filename).expect("no err expected");

    let data16 = input.get();
    let mut sum = 0;
    for i in 0 .. data16.len() {
        sum += data16[i] as u32;
    }
    println!("{} size, {} sum", data16.len(), sum);

}

/*

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            Ok(Box::<MyApp>::default())
        }),
    )
}


#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.image( )
                ui.image(egui::include_image!("ferris.gif"));
            });
        });
    }
} */
