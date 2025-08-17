use eframe::epaint::Color32;
use crate::ui::errors::UiError;
use crate::ui::select_input::parse_value;

#[derive(Clone, Debug)]
pub struct ObjectLightProperties {
    pub kd: [f32; 3],
    pub ks: [f32; 3],
    pub kt: [f32; 3],
    pub luminosity: String,
    pub object_color: Color32,
    pub background: Color32,
}


impl Default for ObjectLightProperties {
    fn default() -> ObjectLightProperties {
        ObjectLightProperties{
            kd: [1.0, 1.0, 1.0],
            ks: [0.0, 0.0, 0.0],
            kt: [0.0, 0.0, 0.0],
            luminosity: String::new(),
            object_color: Color32::from_rgb(192, 192, 192),
            background: Color32::from_rgb(0, 0, 0),
        }
    }
}

impl ObjectLightProperties {
    pub fn parse_luminosity(&mut self) -> Result<i32, UiError>{
        parse_value(
            &self.luminosity,
            UiError::LuminosityError,
            |&n: &i32| n >= 0,
        ).map(|n| n)
    }
}