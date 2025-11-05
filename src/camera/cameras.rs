use nalgebra::{Point3, Vector3};
use crate::camera::FovCamera;
use crate::ui::errors::UiError;


#[derive(Clone, Debug)]
pub struct Cameras{
    pub active_camera_idx: usize,
    pub cameras: Vec<FovCamera>,
}

impl Default for Cameras{
    fn default() -> Cameras{
        Cameras{
            active_camera_idx: 0,
            cameras: Vec::new(),
        }
    }
}

impl Cameras {
    pub fn new(cameras: Vec<FovCamera>) -> Self {
        Self{
            active_camera_idx: 0,
            cameras,
        }
    }
    
    pub fn set_active_camera(&mut self, active_camera_idx: usize)->Result<(), UiError>{
        if active_camera_idx >= self.cameras.len(){
            return Err(UiError::CameraNotPresentedError);
        }
        
        self.active_camera_idx = active_camera_idx;
        Ok(())
    }
    
    pub fn add_camera(&mut self, camera: FovCamera){
        self.cameras.push(camera);
    }
    
    pub fn rotate(&mut self, rotation: &Vector3<f32>){
        if self.cameras.is_empty(){
            return;
        }
        
        self.cameras[self.active_camera_idx].rotate(rotation);
    }
    
    pub fn translate(&mut self, translation: &Vector3<f32>){
        if self.cameras.is_empty(){
            return;
        }
        
        self.cameras[self.active_camera_idx].translate(translation);
    }
    
    pub fn change_fov(&mut self, fov: f32){
        if self.cameras.is_empty(){
            return;
        }
        
        self.cameras[self.active_camera_idx].change_fov(fov);
    }
    
    pub fn camera_position(&self) -> Point3<f32>{
        if self.cameras.is_empty(){
            return Point3::new(0.0, 0.0, 0.0);
        }
        
        self.cameras[self.active_camera_idx].camera_position()
    }
    
    pub fn camera_target(&self) -> Point3<f32>{
        if self.cameras.is_empty(){
            return Point3::new(0.0, 0.0, 0.0);
        }
        
        self.cameras[self.active_camera_idx].camera_target()
    }
    
    pub fn active_camera(&self) -> &FovCamera{
        &self.cameras[self.active_camera_idx]
    }
}
