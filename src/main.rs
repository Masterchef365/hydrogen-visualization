//#![warn(clippy::all, rust_2018_idioms)]
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::{Color32, Stroke};
use ndarray::Array3;
use threegui::Vec3;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.label("Hewwo world :3");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                threegui::ThreeWidget::new("3d")
                    .with_desired_size(ui.available_size())
                    .show(ui, |th| {
                        let paint = th.painter();

                        threegui::utils::grid(paint, 10, 1.0, Stroke::new(1., Color32::DARK_GRAY));

                        let w = 30;
                        let arr = Array3::zeros((w, w, w));
                        draw_array3d_as_points(paint, 1.0, 1.0, &arr);

                    });
            });
        });
    }
}

fn draw_array3d_as_points(paint: &threegui::Painter3D, scale: f32, radius: f32, arr: &Array3<f32>) {
    let origin_3d = Vec3::from_array(
        arr.shape()
            .iter()
            .map(|w| *w as f32)
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap(),
    ) / 2.0;

    let min_dim = *arr.shape().iter().min().unwrap() as f32;

    let scale_factor = scale / min_dim;

    for x in 0..arr.shape()[0] {
        for y in 0..arr.shape()[1] {
            for z in 0..arr.shape()[2] {
                let pt = Vec3::new(x as f32, y as f32, z as f32);
                let mut vect = pt - origin_3d;
                vect *= scale_factor;

                let color = Color32::WHITE;

                paint.circle_filled(vect, radius, color);
            }
        }
    }
}
