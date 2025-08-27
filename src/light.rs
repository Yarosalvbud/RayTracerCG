pub mod lights;

use nalgebra::{Point3, Vector3};

#[derive(Clone, Debug)]
pub struct DistantLight{
    pub origin: Point3<f32>,
    pub intensity: f32,
    pub color: Vec<u8>,
}

impl Default for DistantLight{
    fn default() -> DistantLight {
        DistantLight{
            origin: Point3::new(0.0, 0.0 , 0.0),
            intensity: 0.8,
            color: vec![255, 255, 255]
        }
    }
}

impl DistantLight{
    pub fn vector_to_light(&self, intersection: &Point3<f32>)->Vector3<f32>{
        (self.origin - intersection).normalize()
    }
    
    pub fn translate(&mut self, translation: &Vector3<f32>){
        self.origin += translation;
    }

    pub fn change_light_intensity(&mut self, intensity: f32){
        self.intensity = intensity;
    }

    pub fn change_color(&mut self, color: Vec<u8>){
        self.color = color;
    }
}
