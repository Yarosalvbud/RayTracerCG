use eframe::epaint::Color32;
use crate::ui::errors::UiError;
use crate::ui::move_input::UserMove;
use crate::ui::select_input::parse_value;

#[derive(Debug, Clone)]
pub struct LightProperties {
    pub light_intensity: String,
    pub backlight_intensity: String,
    pub background_const: String,
    pub light_translation: UserMove,
    pub light_color: Color32,
    pub bg_color: Color32,
}

impl Default for LightProperties {
    fn default() -> Self {
        Self {
            light_intensity: String::new(),
            backlight_intensity: String::new(),
            background_const: String::new(),
            light_translation: UserMove::default(),
            light_color: Color32::WHITE,
            bg_color: Color32::WHITE,
        }
    }
}

impl LightProperties {
    pub fn parse_intensity(&self, intensity: &str) -> Result<f32, UiError> {
        parse_value(intensity, UiError::LightIntensityError, |&n: &f32| {
            n >= 0.0 && n <= 1.0
        })
    }
}
