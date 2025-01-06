use std::env;

use std::ffi::OsStr;
use std::time::Instant;
mod input;

use Vec;
// Perform a forward FFT of size 1234
// use eframe::egui;

mod pixmap;

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

    let img = pixmap::create_pixmap(
        data16,
        channels[0].offset,
        channels[0].scale,
        5000,
        512,
        0.1,
    );
    let elapsed = start.elapsed();
    println!(
        "time = {}.{:03}",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
    img.save("output.png").unwrap()
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
