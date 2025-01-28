use std::env;

use eframe::egui::TextureOptions;
use eframe::{egui, emath};
use egui::epaint;
use std::ffi::OsStr;

mod pixmap;
mod input;

const TEXTURE_MAX_SIZE: usize = 16384;

fn load_images(file_name: &OsStr) -> (epaint::image::ColorImage, epaint::image::ColorImage, epaint::image::ColorImage ) {
    let input = input::Input::new(file_name).expect("no err expected");

    let data16 = input.get();
    let channels = &input.channels;
    let fft_size = 512;
    let step_size = (fft_size as f32 / 0.95f32) as usize;

    let width = ((data16.len() - channels[0].offset) / channels[0].scale - fft_size) / step_size;

    let img_l = pixmap::create_pixmap(
        data16,
        channels[0].offset,
        channels[0].scale,
        fft_size,
        step_size,
        width,
    );
    let img_r = pixmap::create_pixmap(
        data16,
        channels[1].offset,
        channels[1].scale,
        fft_size,
        step_size,
        width,
    );

    let img_c = pixmap::create_correlation(
        data16,
        0,
        2,
        fft_size,
        step_size,
        width,
    );

    let im_r = egui::ColorImage::from_rgb(
        [img_l.width() as usize, img_l.height() as usize],
        &img_l.to_vec(),
    );
    let im_l = egui::ColorImage::from_rgb(
        [img_r.width() as usize, img_r.height() as usize],
        &img_r.to_vec(),
    );
    let im_c = egui::ColorImage::from_rgb(
        [img_c.width() as usize, img_c.height() as usize],
        &img_c.to_vec(),
    );
    (im_l, im_r, im_c)
}

fn main() -> eframe::Result {
    let args: Vec<String> = env::args().collect();

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 512.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| {
            let myapp = MyApp {
                texture_r: None,
                texture_l: None,
                texture_c: None,
                other: args[1].clone(),
            };

            Ok(Box::new(myapp))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    texture_l: Option<Vec<egui::TextureHandle>>,
    texture_r: Option<Vec<egui::TextureHandle>>,
    texture_c: Option<Vec<egui::TextureHandle>>,
    other: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.texture_l == None {
                let (im_l, im_r, im_c) = load_images(OsStr::new(&self.other));
                let (num_tex_l, num_tex_r) = (
                    im_l.size[0] / TEXTURE_MAX_SIZE + 1,
                    im_r.size[0] / TEXTURE_MAX_SIZE + 1,
                );
                let mut texture_l = Vec::new();
                let mut texture_r = Vec::new();
                let mut texture_c = Vec::new();
                for i in 0..num_tex_l - 1 {
                    let rect = emath::Rect::from_min_max(
                        emath::pos2((i * TEXTURE_MAX_SIZE) as f32, 0.0),
                        emath::pos2(((i + 1) * TEXTURE_MAX_SIZE) as f32, im_l.size[1] as f32),
                    );

                    texture_l.push(ui.ctx().load_texture(
                        "a",
                        im_l.region(&rect, Some(1.0)),
                        TextureOptions::default(),
                    ));
                    texture_r.push(ui.ctx().load_texture(
                        "a",
                        im_r.region(&rect, Some(1.0)),
                        TextureOptions::default(),
                    ));
                    texture_c.push(ui.ctx().load_texture(
                        "a",
                        im_c.region(&rect, Some(1.0)),
                        TextureOptions::default(),
                    ));
                }
                self.texture_l = Some(texture_l);
                self.texture_r = Some(texture_r);
                self.texture_c = Some(texture_c);
            }

            if let (Some(texture_l), Some(texture_r), Some(texture_c)) = (&self.texture_l, &self.texture_r, &self.texture_c) {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                        for t in texture_l.iter() {
                            ui.image(t);
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                        for t in texture_r.iter() {
                            ui.image(t);
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                        for t in texture_c.iter() {
                            ui.image(t);
                        }
                    });
                });
            }
        });
    }
}
