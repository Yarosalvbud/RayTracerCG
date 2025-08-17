use nalgebra::Vector3;
use crate::controller::Controller;
use crate::ui::base_command::Command;
use crate::ui::errors::UiError;

pub struct ChangeCameraFovCommand<'a> {
    fov: f32,
    controller: &'a mut Controller,
}

impl Command for ChangeCameraFovCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.change_fov(self.fov);
        Ok(())
    }
}

impl<'a> ChangeCameraFovCommand<'a> {
    pub fn new(fov: f32, controller: &'a mut Controller) -> ChangeCameraFovCommand<'a> {
        ChangeCameraFovCommand{
            fov,
            controller
        }
    }
}

pub struct MoveCameraCommand<'a> {
    translation: Vector3<f32>,
    controller: &'a mut Controller,
}

impl Command for MoveCameraCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.move_camera(&self.translation);
        Ok(())
    }
}

impl<'a> MoveCameraCommand<'a> {
    pub fn new(translation: Vector3<f32>, controller: &'a mut Controller) -> MoveCameraCommand<'a> {
        MoveCameraCommand{
            translation,
            controller
        }
    }
}


pub struct RotateCameraCommand<'a> {
    rotation: Vector3<f32>,
    controller: &'a mut Controller,
}

impl Command for RotateCameraCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.rotate_camera(&self.rotation);
        Ok(())
    }
}

impl<'a> RotateCameraCommand<'a> {
    pub fn new(rotation: Vector3<f32>, controller: &'a mut Controller) -> RotateCameraCommand<'a> {
        RotateCameraCommand{
            rotation,
            controller
        }
    }
}
