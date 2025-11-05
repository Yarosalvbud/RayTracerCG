use crate::controller::Controller;
use crate::ui::base_command::Command;
use crate::ui::errors::UiError;
use egui::Color32;
use nalgebra::Vector3;

pub struct RotateCommand<'a> {
    rotation: Vector3<f32>,
    id: usize,
    controller: &'a mut Controller,
}

impl Command for RotateCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.rotate_object(&self.rotation, self.id)
    }
}

impl<'a> RotateCommand<'a> {
    pub fn new(
        rotation: Vector3<f32>,
        id: usize,
        controller: &'a mut Controller,
    ) -> RotateCommand<'a> {
        RotateCommand {
            rotation,
            id,
            controller,
        }
    }
}

pub struct ScaleCommand<'a> {
    scale: Vector3<f32>,
    id: usize,
    controller: &'a mut Controller,
}

impl Command for ScaleCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.scale_object(&self.scale, self.id)
    }
}

impl<'a> ScaleCommand<'a> {
    pub fn new(scale: Vector3<f32>, id: usize, controller: &'a mut Controller) -> ScaleCommand<'a> {
        ScaleCommand {
            scale,
            id,
            controller,
        }
    }
}

pub struct TranslationCommand<'a> {
    translation: Vector3<f32>,
    id: usize,
    controller: &'a mut Controller,
}

impl Command for TranslationCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.move_object(&self.translation, self.id)
    }
}

impl<'a> TranslationCommand<'a> {
    pub fn new(
        translation: Vector3<f32>,
        id: usize,
        controller: &'a mut Controller,
    ) -> TranslationCommand<'a> {
        TranslationCommand {
            translation,
            id,
            controller,
        }
    }
}

pub struct LoadCommand<'a> {
    path: Option<String>,
    controller: &'a mut Controller,
}

impl Command for LoadCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.add_object(self.path.clone())
    }
}

impl<'a> LoadCommand<'a> {
    pub fn new(path: Option<String>, controller: &'a mut Controller) -> LoadCommand<'a> {
        LoadCommand { path, controller }
    }
}

pub struct TexturePropertiesCommand<'a> {
    id: usize,
    object_texture: &'a Option<String>,
    object_normals: &'a Option<String>,
    controller: &'a mut Controller,
}

impl Command for TexturePropertiesCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.add_properties(
            self.id,
            self.object_texture.clone(),
            self.object_normals.clone(),
        )
    }
}

impl<'a> TexturePropertiesCommand<'a> {
    pub fn new(
        id: usize,
        object_texture: &'a Option<String>,
        object_normals: &'a Option<String>,
        controller: &'a mut Controller,
    ) -> TexturePropertiesCommand<'a> {
        TexturePropertiesCommand {
            id,
            object_texture,
            object_normals,
            controller,
        }
    }
}

pub struct LightPropertiesCommand<'a> {
    id: usize,
    kd: [f32; 3],
    ks: [f32; 3],
    kt: [f32; 3],
    color: Color32,
    controller: &'a mut Controller,
}

impl Command for LightPropertiesCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller
            .change_light_properties(self.id, self.kd, self.ks, self.kt, self.color)
    }
}

impl<'a> LightPropertiesCommand<'a> {
    pub fn new(
        id: usize,
        kd: [f32; 3],
        ks: [f32; 3],
        kt: [f32; 3],
        color: Color32,
        controller: &'a mut Controller,
    ) -> LightPropertiesCommand<'a> {
        LightPropertiesCommand {
            id,
            kd,
            ks,
            kt,
            color,
            controller,
        }
    }
}

pub struct RemoveObjectCommand<'a> {
    id: usize,
    controller: &'a mut Controller,
}

impl Command for RemoveObjectCommand<'_> {
    fn execute(&mut self) -> Result<(), UiError> {
        self.controller.remove_object(self.id)
    }
}

impl<'a> RemoveObjectCommand<'a> {
    pub fn new(id: usize, controller: &'a mut Controller) -> RemoveObjectCommand<'a> {
        RemoveObjectCommand { id, controller }
    }
}
