use crate::controller::Controller;
use crate::ui::base_command::Command;
use crate::ui::base_command::camera_commands::{AddCamera, ChangeActiveCamera, ChangeCameraFovCommand, FovCameraPosition, FovCameraTarget, MoveCameraCommand, RotateCameraCommand};
use crate::ui::base_command::light_commands::{
    AddLightCommand, ChangeBackgroundColorCommand, ChangeLightBackIntensityCommand,
    ChangeLightColorCommand, ChangeLightConstCommand, ChangeLightIntensityCommand,
    ChangeLuminosityCommand, LightsPositionsCommand, MoveLightCommand,
};
use crate::ui::base_command::object_commands::{
    LightPropertiesCommand, LoadCommand, RemoveObjectCommand, RotateCommand, ScaleCommand,
    TexturePropertiesCommand, TranslationCommand,
};
use crate::ui::base_command::render_command::RenderCommand;
use crate::ui::camera_input::CameraCommand;
use crate::ui::errors::UiError;
use crate::ui::light_input::LightProperties;
use crate::ui::move_input::{Move, ObjectMove};
use crate::ui::object_light_properties_input::ObjectLightProperties;
use crate::ui::object_load_input::{ObjectProperties, list_files_from_dir};
use crate::ui::select_input::{parse_fov, parse_id};
use eframe::{Frame, egui};
use egui::{Color32, ColorImage, Context, Image, Slider, Vec2};
use nalgebra::Point3;

mod base_command;
mod camera_input;
pub mod errors;
mod light_input;
mod move_input;
mod object_light_properties_input;
mod object_load_input;
mod select_input;

#[derive(Clone, Debug)]
enum PanelSelection {
    ObjectMovement,
    CameraSettings,
    LoadObject,
    LoadProperties,
    LightSettings,
    ObjectLightSettings,
}

#[derive(Clone, Debug)]
pub struct App {
    controller: Controller,
    image: ColorImage,
    show_error: bool,
    error_message: String,
    current_panel: PanelSelection,
    object_properties: ObjectProperties,
    object_move: ObjectMove,
    selected_object: String,
    camera_properties: CameraCommand,
    light_properties: LightProperties,
    object_light_properties: ObjectLightProperties,
    object_list: Vec<String>,
    scene_choice: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            controller: Controller::default(),
            image: ColorImage::new([1920, 1080], vec![Color32::WHITE; 1920 * 1080]),
            object_move: ObjectMove::default(),
            show_error: false,
            error_message: String::new(),
            current_panel: PanelSelection::ObjectMovement,
            object_properties: ObjectProperties::default(),
            selected_object: String::new(),
            camera_properties: CameraCommand::default(),
            light_properties: LightProperties::default(),
            object_light_properties: ObjectLightProperties::default(),
            object_list: Vec::new(),
            scene_choice: "Не выбрано".to_string(),
        }
    }
}

impl App {
    fn object_choice(
        &mut self,
        ui: &mut egui::Ui,
        dir: &str,
        ext: &str,
        label: &str,
        selected_value: &str,
    ) -> Option<String> {
        let files = list_files_from_dir(dir, ext);
        let mut save_to = selected_value.to_string();

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label(label)
                    .selected_text(save_to.clone())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut save_to, "Не выбрано".to_string(), "Не выбрано");
                        for file in files.iter() {
                            ui.selectable_value(&mut save_to, file.clone(), file);
                        }
                    });
            })
        });

        ui.separator();

        if save_to.trim() != "Не выбрано" {
            Some(save_to)
        } else {
            None
        }
    }

    fn read_object(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let stl_choice = self.object_properties.stl_data.clone();
        let stl_dir = std::env::var("UV_DIR").expect("UV_DIR must be set");

        let mut stl = self.object_choice(ui, &stl_dir, "obj", "Объект", &stl_choice);
        if let Some(value) = stl.clone() {
            self.object_properties.stl_data = value.clone();
            stl = Some(format!("{}//{}", stl_dir, value))
        } else {
            self.object_properties.stl_data = "Не выбрано".to_string();
        }

        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Добавить объект"),
            )
            .clicked()
        {
            let mut load = LoadCommand::new(stl, &mut self.controller);
            let result = load.execute();

            if let Err(err) = result {
                self.error_message = err.to_string();
                self.show_error = true;
            } else {
                if self.object_properties.stl_data != "Не выбрано" {
                    self.object_list
                        .push(self.object_properties.stl_data.clone());
                }
                self.render(ctx);
            }
        }

        if self.show_error {
            self.show_error(ctx)
        }
        ui.separator();
    }

    fn read_scene(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let scene_choice = self.scene_choice.clone();
        let scene_dir = std::env::var("SCENE_DIR").expect("SCENE DIR must be set");

        let mut scene = self.object_choice(ui, &scene_dir, "yml", "Сцена", &scene_choice);
        if let Some(value) = scene.clone() {
            self.scene_choice = value.clone();
            scene = Some(format!("{}//{}", scene_dir, value))
        } else {
            self.scene_choice = "Не выбрано".to_string();
        }

        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Загрузить сцену"),
            )
            .clicked()
        {
            let names = self.controller.read_scene(scene);

            if let Err(err) = names {
                self.error_message = err.to_string();
                self.show_error = true;
            } else {
                if self.scene_choice != "Не выбрано" {
                    self.object_list.clear();
                    self.object_list.extend(names.unwrap());
                }else{
                    self.object_list.clear();
                    self.image = ColorImage::new([1920, 1080], vec![Color32::WHITE; 1920 * 1080]);
                }
                self.render(ctx);
            }
        }

        if self.show_error {
            self.show_error(ctx)
        }
        ui.separator();
    }

    fn read_object_properties(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let texture_choice = self.object_properties.texture_data.clone();
        let normal_choice = self.object_properties.normal_map.clone();

        let texture_dir = std::env::var("TEXTURE_DIR").expect("Texture dir must be set");
        let normal_map_dir = std::env::var("NORMAL_MAPS_DIR").expect("NORMAL_MAPS_DIR must be set");

        let mut texture = self.object_choice(ui, &texture_dir, "jpg", "Текстура", &texture_choice);

        if let Some(value) = texture.clone() {
            self.object_properties.texture_data = value.clone();
            texture = Some(format!("{}//{}", texture_dir, value));
        } else {
            self.object_properties.texture_data = "Не выбрано".to_string();
        }

        let mut normal =
            self.object_choice(ui, &normal_map_dir, "jpg", "Карта нормалей", &normal_choice);

        if let Some(value) = normal.clone() {
            self.object_properties.normal_map = value.clone();
            normal = Some(format!("{}//{}", normal_map_dir, value));
        } else {
            self.object_properties.normal_map = "Не выбрано".to_string();
        }

        self.id_input(ui);

        let id = parse_id(&self.selected_object);
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Добавить свойства"),
            )
            .clicked()
        {
            if let Ok(id) = id {
                let mut add_properties =
                    TexturePropertiesCommand::new(id, &texture, &normal, &mut self.controller);

                if let Err(message) = add_properties.execute() {
                    self.error_message = message.to_string();
                    self.show_error = true;
                } else {
                    self.render(ctx);
                }
            }
        }

        if self.show_error {
            self.show_error(ctx)
        }

        ui.separator();
    }

    fn rotation_input(&mut self, ui: &mut egui::Ui) {
        ui.heading("Параметры поворота объекта");

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Rx:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.rotation.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Ry:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.rotation.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Rz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.rotation.dz),
                );
            });
        });
        ui.separator();
    }

    fn scale_input(&mut self, ui: &mut egui::Ui) {
        ui.heading("Параметры масштабирования объекта");

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Sx:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.scale.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Sy:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.scale.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Sz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.scale.dz),
                );
            });
        });
        ui.separator();
    }

    fn move_input(&mut self, ui: &mut egui::Ui) {
        ui.heading("Параметры перемещения объекта");

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Dx:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.translation.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dy:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.translation.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.object_move.translation.dz),
                );
            });
        });
        ui.separator();
    }

    fn camera_move_input(&mut self, ui: &mut egui::Ui) {
        ui.heading("Параметры перемещения камеры");

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Dx:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.camera_properties.translation.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dy:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.camera_properties.translation.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.camera_properties.translation.dz),
                );
            });
        });
        ui.separator();
    }

    fn camera_rotation_input(&mut self, ui: &mut egui::Ui) {
        ui.heading("Параметры поворота камеры");

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Тангаж:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.camera_properties.rotation.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Рыскание:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.camera_properties.rotation.dy),
                );
            });
        });
        ui.separator();
    }

    fn fov_input(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Область видимости:");
            ui.add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::TextEdit::singleline(&mut self.camera_properties.fov),
            );
        });
    }

    fn luminosity_change(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.horizontal(|ui| {
            ui.label("Резкость бликов:");
            ui.add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::TextEdit::singleline(&mut self.object_light_properties.luminosity),
            );
        });

        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Применить преобразование"),
            )
            .clicked()
        {
            let id = parse_id(&self.selected_object);
            let luminosity = self.object_light_properties.parse_luminosity();

            if let Err(message) = id {
                self.error_message = message.to_string();
                self.show_error = true;
            } else {
                let id = id.unwrap();
                if let Err(message) = luminosity {
                    self.error_message = message.to_string();
                    self.show_error = true;
                } else {
                    let luminosity = luminosity.unwrap();

                    let mut change_luminosity_command =
                        ChangeLuminosityCommand::new(id, luminosity, &mut self.controller);

                    if let Err(message) = change_luminosity_command.execute() {
                        self.error_message = message.to_string();
                        self.show_error = true;
                    } else {
                        self.render(ctx);
                    }
                }
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn id_input(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Номер объекта:");
            ui.add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::TextEdit::singleline(&mut self.selected_object),
            );
        });
    }

    fn change_light_intensity(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.vertical(|ui| {
            ui.label("Изменение интенсивности:");
            ui.add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::TextEdit::singleline(&mut self.light_properties.light_intensity),
            );
        });

        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Изменить"),
            )
            .clicked()
        {
            let intensity = self
                .light_properties
                .parse_intensity(&self.light_properties.light_intensity);
            let id = parse_id(&self.selected_object);
            if let Err(message) = id {
                self.error_message = message.to_string();
                self.show_error = true;
            } else {
                let id = id.unwrap();
                if let Err(message) = intensity {
                    self.show_error = true;
                    self.error_message = message.to_string();
                } else {
                    let mut light_intensity_command = ChangeLightIntensityCommand::new(
                        intensity.unwrap(),
                        id,
                        &mut self.controller,
                    );

                    if let Err(message) = light_intensity_command.execute() {
                        self.error_message = message.to_string();
                        self.show_error = true;
                    } else {
                        self.render(ctx);
                    }
                }
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn change_light_back_intensity(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.vertical(|ui| {
            ui.label("Изменение фоновой интенсивности:");
            ui.add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::TextEdit::singleline(&mut self.light_properties.backlight_intensity),
            );
        });

        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Изменить"),
            )
            .clicked()
        {
            let intensity = self
                .light_properties
                .parse_intensity(&self.light_properties.backlight_intensity);
            if let Err(message) = intensity {
                self.show_error = true;
                self.error_message = message.to_string();
            } else {
                let mut change_back_light_intensity_command =
                    ChangeLightBackIntensityCommand::new(intensity.unwrap(), &mut self.controller);
                change_back_light_intensity_command.execute().expect("");
                self.render(ctx);
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn change_light_position(&mut self, ui: &mut egui::Ui) {
        ui.heading("Параметры перемещения света");

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Dx:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.light_properties.light_translation.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dy:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.light_properties.light_translation.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.light_properties.light_translation.dz),
                );
            });
        });
        ui.separator();
    }

    fn change_light_const(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.vertical(|ui| {
            ui.label("Изменение фоновой константы:");
            ui.add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::TextEdit::singleline(&mut self.light_properties.background_const),
            );
        });

        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Изменить"),
            )
            .clicked()
        {
            let ka = self
                .light_properties
                .parse_intensity(&self.light_properties.background_const);
            if let Err(message) = ka {
                self.show_error = true;
                self.error_message = message.to_string();
            } else {
                let mut change_ka_command =
                    ChangeLightConstCommand::new(ka.unwrap(), &mut self.controller);
                change_ka_command.execute().expect("");
                self.render(ctx);
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn change_light_color(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.horizontal(|ui| {
            ui.label("Изменение цвета света:");

            if ui
                .color_edit_button_srgba(&mut self.light_properties.light_color)
                .changed()
            {
                let light_color = self.light_properties.light_color.clone();
                let id = parse_id(&self.selected_object);
                let new_color = vec![light_color[0], light_color[1], light_color[2]];

                if let Err(message) = id {
                    self.show_error = true;
                    self.error_message = message.to_string();
                } else {
                    let id = id.unwrap();
                    let mut color_command =
                        ChangeLightColorCommand::new(new_color, id, &mut self.controller);

                    if let Err(message) = color_command.execute() {
                        self.show_error = true;
                        self.error_message = message.to_string();
                    } else {
                        self.render(ctx);
                    }
                }
            }
        });

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn change_light_bg_color(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.horizontal(|ui| {
            ui.label("Изменение цвета фонового света:");

            if ui
                .color_edit_button_srgba(&mut self.light_properties.bg_color)
                .changed()
            {
                let light_color = self.light_properties.bg_color.clone();
                let new_color = vec![light_color[0], light_color[1], light_color[2]];

                let mut color_command =
                    ChangeBackgroundColorCommand::new(new_color, &mut self.controller);

                color_command.execute().expect("");
                self.render(ctx);
            }
        });

        ui.separator();
    }

    fn set_active_camera(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Установить активную камеру"),
            )
            .clicked()
        {
            let id = parse_id(&self.selected_object);

            if let Err(message) = id {
                self.show_error = true;
                self.error_message = message.to_string();
            }else{
                let id = id.unwrap();

                let mut set_camera_command = ChangeActiveCamera::new(id, &mut self.controller);

                if let Err(message) = set_camera_command.execute() {
                    self.show_error = true;
                    self.error_message = message.to_string();
                }else{
                    self.render(ctx);
                }
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn apply_move(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Применить преобразования"),
            )
            .clicked()
        {
            let move_data = Move::parse_from_string(&self.object_move.translation);
            let scale_data = Move::parse_from_string(&self.object_move.scale);
            let rotation_data = Move::parse_from_string(&self.object_move.rotation);
            let id = parse_id(&self.selected_object);

            if let Err(message) = id {
                self.error_message = message.to_string();
                self.show_error = true;
            } else {
                let id = id.unwrap();

                if move_data.is_err() || scale_data.is_err() || rotation_data.is_err() {
                    let message = UiError::MoveDataFormatError;
                    self.error_message = message.to_string();
                    self.show_error = true;
                }

                if !self.show_error {
                    let mut rotate = RotateCommand::new(
                        rotation_data.unwrap().move_data,
                        id,
                        &mut self.controller,
                    );
                    if let Err(message) = rotate.execute() {
                        self.error_message = message.to_string();
                        self.show_error = true;
                    }

                    let mut scale =
                        ScaleCommand::new(scale_data.unwrap().move_data, id, &mut self.controller);
                    if let Err(message) = scale.execute() {
                        self.error_message = message.to_string();
                        self.show_error = true;
                    }

                    let mut translation = TranslationCommand::new(
                        move_data.unwrap().move_data,
                        id,
                        &mut self.controller,
                    );
                    if let Err(message) = translation.execute() {
                        self.error_message = message.to_string();
                        self.show_error = true;
                    }

                    self.render(ctx);
                }
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn apply_light_move(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Применить преобразования"),
            )
            .clicked()
        {
            let move_data = Move::parse_from_string(&self.light_properties.light_translation);
            let id = parse_id(&self.selected_object);

            if let Err(message) = id {
                self.error_message = message.to_string();
                self.show_error = true;
            } else {
                let id = id.unwrap();

                if move_data.is_err() {
                    let message = UiError::MoveDataFormatError;
                    self.error_message = message.to_string();
                    self.show_error = true;
                }

                if !self.show_error {
                    let mut move_light_command = MoveLightCommand::new(
                        move_data.unwrap().move_data,
                        id,
                        &mut self.controller,
                    );

                    if let Err(message) = move_light_command.execute() {
                        self.error_message = message.to_string();
                        self.show_error = true;
                    } else {
                        self.render(ctx);
                    }
                }
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn add_light(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Добавить источник света"),
            )
            .clicked()
        {
            let mut add_light_command = AddLightCommand::new(&mut self.controller);
            add_light_command.execute().expect("");
            self.render(ctx);
        }
    }

    fn lights_positions(&mut self, ui: &mut egui::Ui) {
        ui.label("Позиции источников света");

        let mut positions: Vec<Point3<f32>> = Vec::new();
        let mut lights_positions =
            LightsPositionsCommand::new(&mut positions, &mut self.controller);
        lights_positions.execute().expect("");

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, position) in positions.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{idx}. x: {:.2}, y: {:.2}, z: {:.2}",
                        position.x, position.y, position.z
                    ));
                });
            }
        });
    }

    fn objects_list(&mut self, ui: &mut egui::Ui) {
        ui.label("Объекты сцены");

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, position) in self.object_list.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{idx}. {position}"));
                });
            }
        });
    }

    fn apply_camera_move(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Применить преобразования"),
            )
            .clicked()
        {
            let move_data = Move::parse_from_string(&self.camera_properties.translation);
            let rotation_data = Move::parse_from_string(&self.camera_properties.rotation);

            if move_data.is_err() || rotation_data.is_err() {
                let message = UiError::MoveDataFormatError;
                self.error_message = message.to_string();
                self.show_error = true;
            }

            if !self.show_error {
                let mut move_camera_command =
                    MoveCameraCommand::new(move_data.unwrap().move_data, &mut self.controller);

                move_camera_command.execute().expect("");

                let mut rotate_camera_command = RotateCameraCommand::new(
                    rotation_data.unwrap().move_data,
                    &mut self.controller,
                );

                rotate_camera_command.execute().expect("");

                self.render(ctx);
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn change_fov(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Изменить область видимости"),
            )
            .clicked()
        {
            let fov = parse_fov(&self.camera_properties.fov);

            if let Err(message) = fov {
                self.error_message = message.to_string();
                self.show_error = true;
            } else {
                let mut change_fov_command =
                    ChangeCameraFovCommand::new(fov.unwrap(), &mut self.controller);

                change_fov_command.execute().expect("");
                self.render(ctx);
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn change_light_params(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.label("Коэффициент диффузного отражения");
        ui.add(Slider::new(&mut self.object_light_properties.kd[0], 0.0..=1.0).text("kd r"));
        ui.add(Slider::new(&mut self.object_light_properties.kd[1], 0.0..=1.0).text("kd g"));
        ui.add(Slider::new(&mut self.object_light_properties.kd[2], 0.0..=1.0).text("kd b"));
        ui.label("Коэффициент зеркального отражения");
        ui.add(Slider::new(&mut self.object_light_properties.ks[0], 0.0..=1.0).text("ks r"));
        ui.add(Slider::new(&mut self.object_light_properties.ks[1], 0.0..=1.0).text("ks g"));
        ui.add(Slider::new(&mut self.object_light_properties.ks[2], 0.0..=1.0).text("ks b"));
        ui.label("Коэффициент пропускания");
        ui.add(Slider::new(&mut self.object_light_properties.kt[0], 0.0..=1.0).text("kt r"));
        ui.add(Slider::new(&mut self.object_light_properties.kt[1], 0.0..=1.0).text("kt g"));
        ui.add(Slider::new(&mut self.object_light_properties.kt[2], 0.0..=1.0).text("kt b"));

        ui.horizontal(|ui| {
            ui.label("Изменение цвета объекта:");
            ui.color_edit_button_srgba(&mut self.object_light_properties.object_color);
        });

        ui.horizontal(|ui| {
            ui.label("Изменение цвета фона");
            if ui
                .color_edit_button_srgba(&mut self.object_light_properties.background)
                .changed()
            {
                self.render(ctx);
            };
        });

        self.id_input(ui);
        ui.separator();
    }

    fn apply_light_properties_change(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Применить преобразования"),
            )
            .clicked()
        {
            let id = parse_id(&self.selected_object);

            if let Err(message) = id {
                self.error_message = message.to_string();
                self.show_error = true;
            } else {
                let mut change_command = LightPropertiesCommand::new(
                    id.unwrap(),
                    self.object_light_properties.kd,
                    self.object_light_properties.ks,
                    self.object_light_properties.kt,
                    self.object_light_properties.object_color,
                    &mut self.controller,
                );

                if let Err(message) = change_command.execute() {
                    self.error_message = message.to_string();
                    self.show_error = true;
                } else {
                    self.render(ctx);
                }
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn add_camera(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Добавить камеру"),
            )
            .clicked(){
            let mut add_camera_command = AddCamera::new(&mut self.controller);
            add_camera_command.execute().expect("");
        }
    }

    fn remove_object(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Удалить объект"),
            )
            .clicked()
        {
            let id = parse_id(&self.selected_object);

            if let Err(message) = id {
                self.error_message = message.to_string();
                self.show_error = true;
            } else {
                let mut remove_object_command =
                    RemoveObjectCommand::new(id.unwrap(), &mut self.controller);

                if let Err(message) = remove_object_command.execute() {
                    self.error_message = message.to_string();
                    self.show_error = true;
                } else {
                    self.object_list.remove(id.unwrap());
                    self.render(ctx);
                }
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }

    fn show_camera_position(&mut self, ui: &mut egui::Ui) {
        let mut origin = Point3::new(0.0, 0.0, 0.0);
        let mut target = Point3::new(0.0, 0.0, 0.0);

        let mut origin_command = FovCameraPosition::new(&mut origin, &mut self.controller);
        origin_command.execute().expect("");

        let mut target_command = FovCameraTarget::new(&mut target, &mut self.controller);
        target_command.execute().expect("");

        ui.label("Позиция камеры:");
        ui.label(format!(
            "x: {:.2} y: {:.2} z: {:.2}",
            origin.x, origin.y, origin.z
        ));
        ui.separator();

        ui.label("Направление камеры:");
        ui.label(format!(
            "x: {:.2} y: {:.2} z: {:.2}",
            target.x, target.y, target.z
        ));
        ui.separator();
    }

    fn show_error(&mut self, ctx: &Context) {
        egui::Window::new("Ошибка")
            .open(&mut self.show_error)
            .resizable(false)
            .movable(false)
            .show(ctx, |ui| {
                ui.label(&self.error_message);
            });
    }

    fn render(&mut self, ctx: &Context) {
        let mut render_command = RenderCommand::new(
            &mut self.image,
            &mut self.controller,
            self.object_light_properties.background,
        );

        if let Err(message) = render_command.execute() {
            self.error_message = message.to_string();
            self.show_error = true;
        }

        if self.show_error {
            self.show_error(ctx);
        }
    }

    fn show_render(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let texture = ctx.load_texture("render", self.image.clone(), Default::default());
        let rect = ui.max_rect();
        let size = rect.size();
        ui.add(Image::new(&texture).fit_to_exact_size(size));
    }

    fn draw_move_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.move_input(ui);
        self.scale_input(ui);
        self.rotation_input(ui);
        self.id_input(ui);
        self.apply_move(ui, ctx);
        self.objects_list(ui);
    }

    fn draw_camera_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.id_input(ui);
        self.set_active_camera(ui, ctx);
        self.camera_move_input(ui);
        self.camera_rotation_input(ui);
        self.apply_camera_move(ui, ctx);
        self.fov_input(ui);
        self.change_fov(ui, ctx);
        self.add_camera(ui);
        self.show_camera_position(ui);
    }

    fn draw_light_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.id_input(ui);
        self.change_light_position(ui);
        self.apply_light_move(ui, ctx);
        self.change_light_intensity(ui, ctx);
        self.change_light_back_intensity(ui, ctx);
        self.change_light_const(ui, ctx);
        self.change_light_color(ui, ctx);
        self.change_light_bg_color(ui, ctx);
        self.add_light(ui, ctx);
        self.lights_positions(ui);
    }

    fn draw_object_settings_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.change_light_params(ui, ctx);
        self.apply_light_properties_change(ui, ctx);
        self.luminosity_change(ui, ctx);
        self.remove_object(ui, ctx);
        self.objects_list(ui);
    }

    fn draw_load_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.read_object(ui, ctx);
        self.read_scene(ui, ctx);
    }

    fn draw_properties_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.read_object_properties(ui, ctx);
        self.objects_list(ui);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Движение объекта").clicked() {
                    self.current_panel = PanelSelection::ObjectMovement;
                }
                if ui.button("Параметры камеры").clicked() {
                    self.current_panel = PanelSelection::CameraSettings;
                }
                if ui.button("Загрузка объектов").clicked() {
                    self.current_panel = PanelSelection::LoadObject;
                }
                if ui.button("Загрузка текстур").clicked() {
                    self.current_panel = PanelSelection::LoadProperties;
                }

                if ui.button("Параметры света").clicked() {
                    self.current_panel = PanelSelection::LightSettings;
                }

                if ui.button("Параметры объекта").clicked() {
                    self.current_panel = PanelSelection::ObjectLightSettings;
                }
            });
        });

        let side_panel_width = ctx.available_rect().width() * 0.3;
        egui::SidePanel::left("Control")
            .resizable(false)
            .min_width(side_panel_width)
            .show(ctx, |ui| match self.current_panel {
                PanelSelection::ObjectMovement => {
                    self.draw_move_panel(ui, ctx);
                }
                PanelSelection::CameraSettings => {
                    self.draw_camera_panel(ui, ctx);
                }
                PanelSelection::LoadObject => {
                    self.draw_load_panel(ui, ctx);
                }
                PanelSelection::LoadProperties => {
                    self.draw_properties_panel(ui, ctx);
                }

                PanelSelection::LightSettings => {
                    self.draw_light_panel(ui, ctx);
                }

                PanelSelection::ObjectLightSettings => {
                    self.draw_object_settings_panel(ui, ctx);
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_render(ui, ctx);
        });
    }
}
