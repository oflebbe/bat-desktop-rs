use std::env;

use eframe::egui::TextureOptions;
use eframe::{egui, emath};
use egui::epaint;
use std::ffi::OsStr;

mod input;
mod pixmap;

fn load_images(
    input: &input::Input,
    start: usize,
    width: usize,
    fft_size: usize,
    overlap: f32,
) -> (
    epaint::image::ColorImage,
    epaint::image::ColorImage,
    epaint::image::ColorImage,
) {
    let data16 = input.get();

    let step_size = (fft_size as f32 * (1.0 - overlap)) as usize;

    // let width = ((data16.len() - channels[0].offset) / channels[0].scale - fft_size) / step_size;

    let image = pixmap::create_pixmap_mono(data16, fft_size, step_size, start, width);

    let im = egui::ColorImage::from_rgb(
        [image.width() as usize, image.height() as usize],
        &image,
    );
    im
}

fn main() -> eframe::Result {
    let args: Vec<String> = env::args().collect();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 512.0]),
        ..Default::default()
    };

    let input = input::Input::new(OsStr::new(&args[1])).expect("no err expected");

    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| {
            let myapp = MyApp {
                input,
                fft_size: FftSizes::F512,
                overlap: 0.8,
                start: 0,
                width: 1200,
            };

            Ok(Box::new(myapp))
        }),
    )
}

struct MyApp {
    input: input::Input,
    fft_size: FftSizes,
    overlap: f32,
    start: usize,
    width: usize,
}

#[derive(PartialEq, Copy, Clone,Default)]
#[repr(i32)]
enum FftSizes {
    F256 = 256,
    #[default]
    F512 = 512,
    F1024 = 1024,
    F2048 = 2048,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.width != (ui.available_width() as usize) {
                self.width = ui.available_width() as usize;
            }
            let old_fft_size = self.fft_size;
            ui.horizontal(|ui| {
                if ui.button("<").clicked() && self.start > self.width / 3 {
                        self.start -= self.width / 3;
                    }
                
                if ui.button(">").clicked() {
                    self.start += self.width / 3;
                }
            });

            egui::ComboBox::from_label("FFT size")
                .selected_text(format!("{}", self.fft_size as usize))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.fft_size, FftSizes::F256, "256");
                    ui.selectable_value(&mut self.fft_size, FftSizes::F512, "512");
                    ui.selectable_value(&mut self.fft_size, FftSizes::F1024, "1024");
                    ui.selectable_value(&mut self.fft_size, FftSizes::F2048, "2048");
                });

            let old_overlap = self.overlap;
            ui.add(
                egui::Slider::new(&mut self.overlap, 0.1..=0.9)
                    .text("f32 demo slider")
                    .smart_aim(false),
            );
            if self.fft_size != old_fft_size || self.overlap != old_overlap {
                println!("overlap: {} {}", old_overlap, self.overlap);
            }

            ui.end_row();
         
            
            ui.spacing_mut().scroll = egui::style::ScrollStyle::thin();

            egui::ScrollArea::both().show_viewport(ui, |ui, rect| {
                ui.set_width(10000.0);
                let im = load_image(
                    &self.input,
                    rect.left() as usize,
                    (rect.right() - rect.left()) as usize,
                    self.fft_size as usize,
                    self.overlap,
                );

                let r = emath::Rect::from_min_max(
                    emath::pos2(0.0, 0.0),
                    emath::pos2(im.size[0] as f32, im.size[1] as f32),
                );

                let texture = ui.ctx().load_texture(
                    "a",
                    im.region(&r, Some(1.0)),
                    TextureOptions::default(),
                );
                

                ui.horizontal(|ui| {
                    ui.add_space(rect.left());
                    ui.image(&texture_l);
                });
            });
        });
    }
}
