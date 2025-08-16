use nalgebra::{Point3, Vector3};

#[derive(Clone, Debug)]
pub struct DistantLight{
    pub origin: Point3<f32>,
    pub intensity: f32,
    pub back_intensity: f32,
    pub ka: f32,
    pub color: Vec<u8>,
}

impl Default for DistantLight{
    fn default() -> DistantLight {
        DistantLight{
            origin: Point3::new(4.07, -1.0 , 5.9),
            intensity: 0.8,
            back_intensity: 0.2,
            ka: 0.3,
            color: vec![255, 180, 100]
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
}
