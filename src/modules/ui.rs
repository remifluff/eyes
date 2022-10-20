use nannou::{prelude::Update, prelude::*, window::Window, Draw};
use nannou_egui::{self, egui, Egui};

use crate::Settings;

pub struct UI {
    pub egui: Egui,
}
impl UI {
    pub fn new(window: &Window) -> UI {
        UI {
            egui: Egui::from_window(&window),
        }
    }
    pub fn update(&mut self, update: Update, settings: &mut Settings) {
        let egui = &mut self.egui;
        egui.set_elapsed_time(update.since_start);
        let ctx = egui.begin_frame();
        egui::Window::new("Workshop window").show(&ctx, |ui| {
            let mut changed = false;
            changed |= ui
                .add(
                    egui::Slider::new(
                        &mut settings.min_radius,
                        0.0..=20.0,
                    )
                    .text("min radius"),
                )
                .changed();
            changed |= ui
                .add(
                    egui::Slider::new(
                        &mut settings.max_radius,
                        0.0..=200.0,
                    )
                    .text("max radius"),
                )
                .changed();
            changed |= ui
                .add(
                    egui::Slider::new(
                        &mut settings.circle_count,
                        0..=2000,
                    )
                    .text("circle count"),
                )
                .changed();
            changed |= ui.button("Generate").clicked();
            // if changed {
            //     *circles = generate_circles(settings);
            // }
        });
    }

    pub fn draw(&self, frame: &Frame) {
        self.egui.draw_to_frame(&frame).unwrap();
    }
}
