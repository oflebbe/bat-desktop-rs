use std::env;

use std::ffi::OsStr;
use std::time::Instant;
mod input;
use eframe::egui;
use egui::epaint;

mod pixmap;

fn load_images(file_name: &OsStr) -> (epaint::image::ColorImage, epaint::image::ColorImage) {
    let input = input::Input::new(file_name).expect("no err expected");

    let data16 = input.get();
    let channels = &input.channels;

    let img_l = pixmap::create_pixmap(
        data16,
        channels[0].offset + 1000000,
        channels[0].scale,
        5000,
        512,
        0.1,
    );

    let img_r = pixmap::create_pixmap(
        data16,
        channels[1].offset + 1000000,
        channels[1].scale,
        5000,
        512,
        0.1,
    );

    let im_r = egui::ColorImage::from_rgb(
        [img_l.width() as usize, img_l.height() as usize],
        &img_l.to_vec(),
    );
    let im_l = egui::ColorImage::from_rgb(
        [img_r.width() as usize, img_r.height() as usize],
        &img_r.to_vec(),
    );
    (im_l, im_r)
}

fn main() -> eframe::Result {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| {
            let myapp = MyApp {
                texture_r: None,
                texture_l: None,
                other: args[1].clone(),
            };

            Ok(Box::new(myapp))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    texture_l: Option<egui::TextureHandle>,
    texture_r: Option<egui::TextureHandle>,

    other: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.texture_l == None {
                let (im_l, im_r) = load_images(OsStr::new(&self.other));
                self.texture_l = Some(ui.ctx().load_texture(
                    "bat034",
                    im_l,
                    egui::TextureOptions::default(),
                ));
                self.texture_r = Some(ui.ctx().load_texture(
                    "bat034",
                    im_r,
                    egui::TextureOptions::default(),
                ));
            }

            if let (Some(texture_l), Some(texture_r)) = (&self.texture_l, &self.texture_r) {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.image(texture_l);
                    ui.image(texture_r);
                });
            }
        });
    }
}
