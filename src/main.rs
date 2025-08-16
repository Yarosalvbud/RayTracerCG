mod light;
mod camera;
pub mod polygon;
mod ray_tracer;
pub mod texture;
mod ui;
mod controller;
use crate::ui::App;

fn main() {
    let options = eframe::NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "Figures app",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles = [
                (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Body, egui::FontId::new(25.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Button, egui::FontId::new(25.0, egui::FontFamily::Proportional)),
            ].into();
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_visuals(egui::Visuals::light());

            Ok(Box::new(App::default()))
        }),
    ).expect("Ошибка при запуске");
}