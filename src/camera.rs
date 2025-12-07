pub mod cameras;

use nalgebra::{Matrix4, Point3, Vector3};


#[derive(Clone, Debug)]
pub struct FovCamera {
    pub origin: Point3<f32>,
    pub target: Point3<f32>,
    pub world_up: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
}

impl Default for FovCamera {
    fn default() -> FovCamera {
        let mut cam: FovCamera = FovCamera {
            origin: Point3::new(8.73, -7.01, 5.24),
            target: Point3::new(9.35, -7.74, 5.53),
            world_up: Vector3::new(0.0, 0.0, 1.0),
            yaw: -40.0,
            pitch: 30.0,
            fov: 39.0,
        };
        cam.rotate(&Vector3::zeros());

        cam
    }
}

impl FovCamera {
    pub fn new(origin: Point3<f32>, target: Point3<f32>, yaw: f32, pitch: f32, fov: f32) -> FovCamera {
        FovCamera{
            origin,
            target,
            world_up: Vector3::new(0.0, 0.0, 1.0),
            yaw,
            pitch, 
            fov
        }
    }
    
    pub fn translate(&mut self, translation: &Vector3<f32>) {
        let front = (self.target - self.origin).normalize();
        let right = front.cross(&self.world_up).normalize();
        let up = right.cross(&front).normalize();

        self.origin += translation.x * right;
        self.origin += translation.y * front;
        self.origin += translation.z * up;
    }

    pub fn rotate(&mut self, rotation: &Vector3<f32>) {
        self.yaw += rotation[1];
        self.pitch += rotation[0];

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        let mut front = Vector3::zeros();

        front.x = f32::cos(self.yaw.to_radians()) * f32::cos(self.pitch.to_radians());
        front.z = f32::sin(self.pitch.to_radians());
        front.y = f32::sin(self.yaw.to_radians()) * f32::cos(self.pitch.to_radians());
        front = front.normalize();
        
        self.target = self.origin + front;
    }

    pub fn camera_to_world(&self, world_up: Option<&Vector3<f32>>) -> Matrix4<f32> {
        if let Some(world_up) = world_up {
            Matrix4::face_towards(&self.origin, &self.target, &world_up)
        }else{
            Matrix4::face_towards(&self.origin, &self.target, &self.world_up)
        }
    }
    
    pub fn change_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
    
    pub fn camera_position(&self) -> Point3<f32> {
        self.origin 
    }
    
    pub fn camera_target(&self) -> Point3<f32> {
        self.target
    }
}
