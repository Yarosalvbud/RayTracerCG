use nalgebra::Vector3;
use crate::controller::Controller;
use crate::ui::base_command::Command;
use crate::ui::errors::UiError;

pub struct ChangeLightIntensityCommand<'a> {
    intensity: f32,
    controller: &'a mut Controller,
}

impl Command for ChangeLightIntensityCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.change_light_intensity(self.intensity);
        Ok(())
    }
}

impl<'a> ChangeLightIntensityCommand<'a> {
    pub fn new(intensity: f32, controller: &'a mut Controller) -> ChangeLightIntensityCommand<'a> {
        ChangeLightIntensityCommand { controller, intensity }
    }
}

pub struct ChangeLightBackIntensityCommand<'a> {
    intensity: f32,
    controller: &'a mut Controller,
}

impl Command for ChangeLightBackIntensityCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.change_light_back_intensity(self.intensity);
        Ok(())
    }
}

impl<'a> ChangeLightBackIntensityCommand<'a> {
    pub fn new(intensity: f32, controller: &'a mut Controller) -> ChangeLightBackIntensityCommand<'a> {
        ChangeLightBackIntensityCommand { controller, intensity }
    }
}

pub struct ChangeLightConstCommmand<'a> {
    ka: f32,
    controller: &'a mut Controller,
}

impl Command for ChangeLightConstCommmand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.change_back_const(self.ka);
        Ok(())
    }
}

impl<'a> ChangeLightConstCommmand<'a> {
    pub fn new(ka: f32, controller: &'a mut Controller) -> ChangeLightConstCommmand<'a> {
        ChangeLightConstCommmand { controller, ka }
    }
}

pub struct ChangeLightColorCommand<'a> {
    color: Vec<u8>,
    controller: &'a mut Controller,
}

impl Command for ChangeLightColorCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.change_light_color(&self.color);
        Ok(())
    }
}

impl<'a> ChangeLightColorCommand<'a> {
    pub fn new(color: Vec<u8>, controller: &'a mut Controller) -> ChangeLightColorCommand<'a> {
        ChangeLightColorCommand { controller, color }
    }
}

pub struct MoveLightCommand<'a> {
    translation: Vector3<f32>,
    controller: &'a mut Controller,
}

impl Command for MoveLightCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.move_light(&self.translation);
        Ok(())
    }
}

impl<'a> MoveLightCommand<'a> {
    pub fn new(translation: Vector3<f32>, controller: &'a mut Controller) -> MoveLightCommand<'a> {
        MoveLightCommand { controller, translation }
    }
}
