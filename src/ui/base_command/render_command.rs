use eframe::epaint::Color32;
use egui::ColorImage;
use crate::controller::Controller;
use crate::ui::base_command::Command;
use crate::ui::errors::UiError;

pub struct RenderCommand<'a> {
    image: &'a mut ColorImage,
    background_color: Color32,
    controller: &'a mut Controller,
}

impl Command for RenderCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.render(self.image, self.background_color);
        Ok(())
    }
}

impl<'a> RenderCommand<'a> {
    pub fn new(image: &'a mut ColorImage, controller: &'a mut Controller, background_color: Color32) -> RenderCommand<'a> {
        RenderCommand { image, background_color, controller }
    }
}