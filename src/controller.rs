use crate::polygon::polygon_mesh_builder::PolygonMeshBuilder;
use crate::ray_tracer::Ray;
use crate::ui::errors::UiError;
use eframe::epaint::Color32;
use egui::ColorImage;
use nalgebra::{Point3, Vector3};
use crate::camera::FovCamera;
use crate::scene::Scene;

#[derive(Clone, Debug)]
pub struct Controller {
    scene: Scene
}

const DEFAULT_KD: [f32; 3] = [1.0, 1.0, 1.0];
const DEFAULT_KS: [f32; 3] = [0.0, 0.0, 0.0];
const DEFAULT_KT: [f32; 3] = [0.0, 0.0, 0.0];
const DEFAULT_COLOR: [u8; 3] = [192, 192, 192];
const DEFAULT_LUMINOSITY: i32 = 50;

impl Default for Controller {
    fn default() -> Controller {
        Controller {
            scene: Scene::default(),
        }
    }
}

impl Controller {
    pub fn move_object(&mut self, translation: &Vector3<f32>, id: usize) -> Result<(), UiError> {
        if id >= self.scene.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.scene.meshes.translate(translation, id);
        Ok(())
    }

    pub fn rotate_object(&mut self, rotation: &Vector3<f32>, id: usize) -> Result<(), UiError> {
        if id >= self.scene.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.scene.meshes.rotation(rotation, id);
        Ok(())
    }

    pub fn scale_object(&mut self, scale: &Vector3<f32>, id: usize) -> Result<(), UiError> {
        if id >= self.scene.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        if scale.x == 0.0 || scale.y == 0.0 || scale.z == 0.0 {
            return Err(UiError::ScaleError);
        }

        self.scene.meshes.scale(scale, id);
        Ok(())
    }

    pub fn remove_object(&mut self, id: usize) -> Result<(), UiError> {
        if id >= self.scene.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.scene.meshes.remove_object(id);
        Ok(())
    }

    pub fn move_camera(&mut self, translation: &Vector3<f32>) {
        self.scene.cameras.translate(translation);
    }

    pub fn rotate_camera(&mut self, rotation: &Vector3<f32>) {
        self.scene.cameras.rotate(rotation);
    }

    pub fn change_fov(&mut self, fov: f32) {
        self.scene.cameras.change_fov(fov);
    }

    pub fn change_light_intensity(
        &mut self,
        light_intensity: f32,
        id: usize,
    ) -> Result<(), UiError> {
        self.scene.lights.change_light_intensity(light_intensity, id)?;

        Ok(())
    }

    pub fn change_light_back_intensity(&mut self, intensity: f32) {
        self.scene.lights.change_light_back_intensity(intensity);
    }

    pub fn change_object_luminosity(&mut self, id: usize, luminosity: i32) -> Result<(), UiError> {
        if id >= self.scene.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.scene.meshes.set_luminosity(id, luminosity);

        Ok(())
    }

    pub fn change_back_const(&mut self, ka: f32) {
        self.scene.lights.change_ka(ka);
    }

    pub fn move_light(&mut self, translation: &Vector3<f32>, id: usize) -> Result<(), UiError> {
        self.scene.lights.translate(translation, id)?;

        Ok(())
    }

    pub fn change_light_color(&mut self, color: &Vec<u8>, id: usize) -> Result<(), UiError> {
        self.scene.lights.change_color(color.clone(), id)?;

        Ok(())
    }

    pub fn add_light(&mut self) {
        self.scene.lights.new_light();
    }

    pub fn change_bg_color(&mut self, color: &Vec<u8>) {
        self.scene.lights.change_light_back_color(color.clone());
    }

    pub fn render(&mut self, image: &mut ColorImage, bg_color: Color32)-> Result<(), UiError> {
        if self.scene.cameras.cameras.is_empty() {
            return Err(UiError::RenderError);
        }
        
        let bg_color = vec![bg_color.r(), bg_color.g(), bg_color.b()];
        Ray::render(
            &self.scene.meshes,
            self.scene.cameras.active_camera(),
            &self.scene.lights,
            image,
            bg_color,
            7
        );
        
        Ok(())
    }

    pub fn change_light_properties(
        &mut self,
        id: usize,
        kd: [f32; 3],
        ks: [f32; 3],
        kt: [f32; 3],
        color: Color32,
    ) -> Result<(), UiError> {
        if kd[0] + kt[0] + ks[0] > 1.0 || kd[1] + kt[1] + ks[1] > 1.0 || kd[2] + kt[2] + ks[2] > 1.0
        {
            return Err(UiError::ColorPropertiesError);
        }

        if id >= self.scene.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.scene.meshes.set_kd(id, kd.to_vec());
        self.scene.meshes.set_ks(id, ks.to_vec());
        self.scene.meshes.set_kt(id, kt.to_vec());
        self.scene.meshes
            .set_color(id, vec![color[0], color[1], color[2]]);

        Ok(())
    }

    pub fn add_object(&mut self, object_path: Option<String>) -> Result<(), UiError> {
        let mut builder = PolygonMeshBuilder::new();
        builder.build_polygons(object_path)?;
        let mesh = builder.build_object(
            DEFAULT_COLOR.to_vec(),
            DEFAULT_KD.to_vec(),
            DEFAULT_KS.to_vec(),
            DEFAULT_KT.to_vec(),
            DEFAULT_LUMINOSITY,
        );
        self.scene.meshes.add(mesh);

        Ok(())
    }

    pub fn add_properties(
        &mut self,
        id: usize,
        object_texture: Option<String>,
        object_normals: Option<String>,
    ) -> Result<(), UiError> {
        if id >= self.scene.meshes.length() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.scene.meshes.set_texture(id, object_texture)?;
        self.scene.meshes.set_normal_map(id, object_normals)?;

        Ok(())
    }

    pub fn lights_positions(&self, positions: &mut Vec<Point3<f32>>) {
        *positions = self.scene.lights.lights_positions();
    }

    pub fn camera_position(&self, position: &mut Point3<f32>) {
        *position = self.scene.cameras.camera_position();
    }

    pub fn add_camera(&mut self){
        let camera = FovCamera::default();
        self.scene.cameras.add_camera(camera);
    }

    pub fn set_active_camera(&mut self, id: usize)->Result<(), UiError> {
        self.scene.cameras.set_active_camera(id)?;

        Ok(())
    }

    pub fn camera_target(&self, target: &mut Point3<f32>) {
        *target = self.scene.cameras.camera_target()
    }
    
    pub fn read_scene(&mut self, path: Option<String>)-> Result<Vec<String>, Box<dyn std::error::Error>> {
        if let None = path{
            return Ok(Vec::new());
        }
        
        let (scene, names) = Scene::loading_scene(&path.unwrap())?;
        self.scene = scene;

        for camera in self.scene.cameras.cameras.iter_mut() {
            camera.rotate(&Vector3::zeros());
        }
        Ok(names)
    }
}
