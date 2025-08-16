use crate::ui::errors::UiError;
use crate::ui::move_commands::UserMove;
use crate::ui::select_command::parse_value;

#[derive(Debug, Clone)]
pub struct LightColorInput {
    pub r: String,
    pub g: String,
    pub b: String,
}

#[derive(Debug, Clone)]
pub struct LightColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl LightColor {
    pub fn new(r: u8, g: u8, b: u8) -> LightColor {
        LightColor { r, g, b }
    }
}

#[derive(Debug, Clone)]
pub struct LightProperties {
    pub light_intensity: String,
    pub backlight_intensity: String,
    pub background_const: String,
    pub light_color: LightColorInput,
    pub light_translation: UserMove,
}

impl Default for LightColorInput {
    fn default() -> Self {
        Self {
            r: String::new(),
            g: String::new(),
            b: String::new(),
        }
    }
}

impl Default for LightProperties {
    fn default() -> Self {
        Self {
            light_intensity: String::new(),
            backlight_intensity: String::new(),
            background_const: String::new(),
            light_color: LightColorInput::default(),
            light_translation: UserMove::default(),
        }
    }
}

impl LightProperties {
    pub fn parse_intensity(&self, intensity: &str) -> Result<f32, UiError> {
        parse_value(intensity, UiError::LightIntensityError, |&n: &f32| {
            n >= 0.0 && n <= 1.0
        })
    }

    pub fn parse_color(&self) -> Result<LightColor, UiError> {
        let r = parse_value(&self.light_color.r, UiError::ColorError, |&n: &i32| {
            n >= 0 && n <= 255
        })?;

        let g = parse_value(&self.light_color.g, UiError::ColorError, |&n: &i32| {
            n >= 0 && n <= 255
        })?;

        let b = parse_value(&self.light_color.b, UiError::ColorError, |&n: &i32| {
            n >= 0 && n <= 255
        })?;
        
        Ok(LightColor::new(r as u8, g as u8, b as u8))
    }
}
