use nalgebra::{Point3, Vector3};
use crate::light::DistantLight;
use crate::ui::errors::UiError;

#[derive(Clone, Debug)]
pub struct Lights{
    pub back_intensity: f32,
    pub ka: f32,
    pub back_color: Vec<u8>,
    pub lights: Vec<DistantLight>,
}

impl Default for Lights{
    fn default() -> Self{
        Self{
            back_intensity: 0.01,
            ka: 0.2,
            back_color: vec![255, 255, 255],
            lights: Vec::new(),
        }
    }
}

impl Lights{
    pub fn new(back_intensity: f32, ka: f32, back_color: Vec<u8>, lights: Vec<DistantLight>) -> Self {
        Lights{
            back_intensity,
            ka,
            back_color,
            lights
        }
    }
    
    pub fn translate(&mut self, translation: &Vector3<f32>, id: usize)-> Result<(), UiError>{
        if id >= self.lights.len(){
            return Err(UiError::ObjectNotFoundError);
        }
        
        self.lights[id].translate(translation);
        
        Ok(())
    }

    pub fn change_light_intensity(&mut self, intensity: f32, id: usize)-> Result<(), UiError>{
        if id >= self.lights.len(){
            return Err(UiError::ObjectNotFoundError);
        }
        
        self.lights[id].change_light_intensity(intensity);
        
        Ok(())
    }

    pub fn change_color(&mut self, color: Vec<u8>, id: usize)-> Result<(), UiError>{
        if id >= self.lights.len(){
            return Err(UiError::ObjectNotFoundError);
        }
        
        self.lights[id].change_color(color);
        
        Ok(())
    }
    
    pub fn change_ka(&mut self, ka: f32){
        self.ka = ka;
    }
    
    pub fn change_light_back_intensity(&mut self, intensity: f32){
        self.back_intensity = intensity;
    }
    
    pub fn change_light_back_color(&mut self, color: Vec<u8>){
        self.back_color = color;
    }
    
    pub fn new_light(&mut self){
        self.lights.push(DistantLight::default());
    }

    pub fn lights_positions(&self)->Vec<Point3<f32>>{
        let mut positions: Vec<Point3<f32>> = Vec::new();

        for light in self.lights.iter(){
            positions.push(light.origin);
        }

        positions
    }

    pub fn add(&mut self, light: DistantLight){
        self.lights.push(light);
    }
}
