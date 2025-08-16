use crate::controller::Controller;
use crate::ui::move_commands::{Move, UserMove};
use crate::ui::object_load_commands::{ObjectProperties, list_files_from_dir};
use crate::ui::select_command::{parse_fov, parse_id};
use eframe::{Frame, egui};
use egui::{Color32, ColorImage, Context, Image, TextBuffer, Vec2};
use std::ops::Deref;
use crate::ui::errors::UiError;
use crate::ui::light_commands::LightProperties;

pub mod errors;
mod move_commands;
mod object_load_commands;
mod select_command;
mod light_commands;

#[derive(Clone, Debug)]
enum PanelSelection {
    ObjectMovement,
    CameraSettings,
    LoadObject,
    LoadProperties,
    LightSettings,
}

#[derive(Clone, Debug)]
pub struct App {
    controller: Controller,
    image: ColorImage,
    rotation_params: UserMove,
    scale_params: UserMove,
    translation_params: UserMove,
    show_error: bool,
    error_message: String,
    current_panel: PanelSelection,
    object_properties: ObjectProperties,
    selected_object: String,
    camera_translation: UserMove,
    camera_rotation: UserMove,
    camera_fov: String,
    light_properties: LightProperties,
}

impl Default for App {
    fn default() -> Self {
        Self {
            controller: Controller::default(),
            image: ColorImage::new([1920, 1080], vec![Color32::WHITE; 1920 * 1080]),
            rotation_params: UserMove::default(),
            scale_params: UserMove::default(),
            translation_params: UserMove::default(),
            show_error: false,
            error_message: String::new(),
            current_panel: PanelSelection::ObjectMovement,
            object_properties: ObjectProperties::default(),
            selected_object: String::new(),
            camera_translation: UserMove::default(),
            camera_rotation: UserMove::default(),
            camera_fov: String::new(),
            light_properties: LightProperties::default(),
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
                ui.label(label);
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
        let stl_dir = "../data/stl_models".to_string();

        let mut stl = self.object_choice(
            ui,
            &stl_dir,
            "stl",
            "Объект",
            &stl_choice,
        );
        if let Some(value) = stl.clone() {
            self.object_properties.stl_data = value.clone();
            stl = Some(format!("{}//{}", stl_dir, value))
        }else{
            self.object_properties.stl_data = "Не выбрано".to_string();
        }

        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Добавить объект"),
            )
            .clicked()
        {
            let result = self.controller.add_object(stl);

            if let Err(err) = result {
                self.error_message = err.to_string();
                self.show_error = true;
            } else {
                self.render();
            }
        }

        if self.show_error {
            self.show_error(ctx)
        }
        ui.separator();
    }
    
    fn read_object_properties(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let texture_choice =  self.object_properties.texture_data.clone();
        let normal_choice = self.object_properties.normal_map.clone();
        let uv_choice = self.object_properties.uv.clone();
        
        let texture_dir = "../data/textures".to_string();
        let normal_map_dir =  "../data/normal_maps".to_string();
        let uv_dir = "../data/uv_unwrap".to_string();
        
        let mut texture =  self.object_choice(
            ui,
            &texture_dir,
            "jpg",
            "Текстура",
            &texture_choice,
        );

        if let Some(value) = texture.clone() {
            self.object_properties.texture_data = value.clone();
            texture = Some(format!("{}//{}", texture_dir, value));
        }else{
            self.object_properties.texture_data = "Не выбрано".to_string();;
        }
        
        let mut normal = self.object_choice(
            ui,
            &normal_map_dir,
            "jpg",
            "Карта нормалей",
            &normal_choice,
        );

        if let Some(value) = normal.clone() {
            self.object_properties.normal_map = value.clone();
            normal = Some(format!("{}//{}", normal_map_dir, value));
        }else{
            self.object_properties.normal_map = "Не выбрано".to_string();
        }
        
        let mut uv = self.object_choice(
            ui,
            &uv_dir,
            "obj",
            "UV Развертка",
            &uv_choice,
        ); 
        
        if let Some(value) = uv.clone() {
            self.object_properties.uv = value.clone();
            uv = Some(format!("{}//{}", uv_dir, value));
        }else{
            self.object_properties.uv = "Не выбрано".to_string();
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
                if let Some(texture) = texture {
                    if let Some(uv) = uv {
                        if let Err(message) = self.controller.add_properties(id, texture, normal, uv) {
                            self.error_message = message.to_string();
                            self.show_error = true;
                        }else{
                            self.render();
                        }
                    } else {
                        self.error_message = UiError::NoUnwrapError.to_string();
                        self.show_error = true;
                    }
                } else {
                    self.error_message = UiError::NoTextureError.to_string();
                    self.show_error = true;
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
                    egui::TextEdit::singleline(&mut self.rotation_params.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Ry:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.rotation_params.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Rz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.rotation_params.dz),
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
                    egui::TextEdit::singleline(&mut self.scale_params.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Sy:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.scale_params.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Sz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.scale_params.dz),
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
                    egui::TextEdit::singleline(&mut self.translation_params.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dy:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.translation_params.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.translation_params.dz),
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
                    egui::TextEdit::singleline(&mut self.camera_translation.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dy:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.camera_translation.dy),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Dz:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.camera_translation.dz),
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
                    egui::TextEdit::singleline(&mut self.camera_rotation.dx),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Рыскание:");
                ui.add_sized(
                    Vec2::new(ui.available_width(), 33.0),
                    egui::TextEdit::singleline(&mut self.camera_rotation.dy),
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
                egui::TextEdit::singleline(&mut self.camera_fov),
            );
        });
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
            let intensity = self.light_properties.parse_intensity(&self.light_properties.light_intensity);
            if let Err(message) = intensity {
                self.show_error = true;
                self.error_message = message.to_string();
            }else{
                self.controller.change_light_intensity(intensity.unwrap());
                self.render();
            }
        }
        
        if self.show_error{
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
            let intensity = self.light_properties.parse_intensity(&self.light_properties.backlight_intensity);
            if let Err(message) = intensity {
                self.show_error = true;
                self.error_message = message.to_string();
            }else{
                self.controller.change_light_back_intensity(intensity.unwrap());
                self.render();
            }
        }

        if self.show_error{
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
            let ka = self.light_properties.parse_intensity(&self.light_properties.background_const);
            if let Err(message) = ka {
                self.show_error = true;
                self.error_message = message.to_string();
            }else{
                self.controller.change_back_const(ka.unwrap());
                self.render();
            }
        }

        if self.show_error{
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
            let move_data = Move::parse_from_string(&self.translation_params);
            let scale_data = Move::parse_from_string(&self.scale_params);
            let rotation_data = Move::parse_from_string(&self.rotation_params);
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
                
                if !self.show_error{
                    self.controller.rotate_object(&rotation_data.unwrap().move_data, id);
                    self.controller.scale_object(&scale_data.unwrap().move_data, id);
                    self.controller.move_object(&move_data.unwrap().move_data, id);

                    self.render();
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
            if move_data.is_err(){
                let message = UiError::MoveDataFormatError;
                self.error_message = message.to_string();
                self.show_error = true;
            }

            if !self.show_error{
                self.controller.move_light(&move_data.unwrap().move_data);
                self.render();
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }
    
    fn apply_camera_move(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Применить преобразования"),
            )
            .clicked()
        {
            let move_data = Move::parse_from_string(&self.camera_translation);
            let rotation_data = Move::parse_from_string(&self.camera_rotation);
            

            if move_data.is_err() || rotation_data.is_err(){
                let message = UiError::MoveDataFormatError;
                self.error_message = message.to_string();
                self.show_error = true;
            }

            if !self.show_error{
                self.controller.move_camera(&move_data.unwrap().move_data);
                self.controller.rotate_camera(&rotation_data.unwrap().move_data);

                self.render();
            }
        }

        if self.show_error {
            self.show_error(ctx);
        }

        ui.separator();
    }
    
    fn change_fov(&mut self, ui: &mut egui::Ui, ctx: &Context){
        if ui
            .add_sized(
                Vec2::new(ui.available_width(), 33.0),
                egui::Button::new("Изменить область видимости"),
            )
            .clicked()
        {
            let fov = parse_fov(&self.camera_fov);
            
            if let Err(message) = fov {
                self.error_message = message.to_string();
                self.show_error = true;
            }else{
                self.controller.change_fov(fov.unwrap());
                self.render();
            }
        }
        
        if self.show_error {
            self.show_error(ctx);
        }
        
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

    fn render(&mut self) {
        self.controller.render(&mut self.image);
    }

    fn show_render(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let texture = ctx.load_texture("render", self.image.clone(), Default::default());
        ui.add(
            Image::new(&texture).fit_to_exact_size(Vec2::new(ui.available_width(), ui.available_height())),
        );
    }

    fn draw_move_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.move_input(ui);
        self.scale_input(ui);
        self.rotation_input(ui);
        self.id_input(ui);
        self.apply_move(ui, ctx);
    }
    
    fn draw_camera_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.camera_move_input(ui);
        self.camera_rotation_input(ui);
        self.apply_camera_move(ui, ctx);
        self.fov_input(ui);
        self.change_fov(ui, ctx);
    }
    
    fn draw_light_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.change_light_position(ui);
        self.apply_light_move(ui, ctx);
        self.change_light_intensity(ui, ctx);
        self.change_light_back_intensity(ui, ctx);
        self.change_light_const(ui, ctx);
    }

    fn draw_load_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.read_object(ui, ctx);
    }
    
    fn draw_properties_panel(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.read_object_properties(ui, ctx);
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
                if ui.button("Загрузка объекта").clicked() {
                    self.current_panel = PanelSelection::LoadObject;
                }
                if ui.button("Загрузка текстур").clicked() {
                    self.current_panel = PanelSelection::LoadProperties;
                }
                
                if ui.button("Параметры света").clicked() {
                    self.current_panel = PanelSelection::LightSettings;
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
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_render(ui, ctx);
        });
    }
}
