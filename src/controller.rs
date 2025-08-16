use crate::camera::FovCamera;
use crate::light::DistantLight;
use crate::polygon::file_reader::{loading_stl_model, loading_uv_obj_data};
use crate::polygon::polygon_mesh::PolygonMesh;
use crate::polygon::polygon_mesh::polygon_meshes::PolygonMeshes;
use crate::ray_tracer::Ray;
use crate::texture::Texture;
use crate::ui::errors::UiError;
use egui::ColorImage;
use nalgebra::Vector3;

#[derive(Clone, Debug)]
pub struct Controller {
    meshes: PolygonMeshes,
    lights: Vec<DistantLight>,
    fov_camera: FovCamera,
}

const DEFAULT_KD: [f32; 3] = [1.0, 1.0, 1.0];
const DEFAULT_KS: [f32; 3] = [0.0, 0.0, 0.0];
const DEFAULT_KT: [f32; 3] = [0.0, 0.0, 0.0];
const DEFAULT_COLOR: [u8; 3] = [192, 192, 192];
const DEFAULT_LUMINOSITY: i32 = 50;

impl Default for Controller {
    fn default() -> Controller {
        let mut meshes = PolygonMeshes::default(); //todo(Убрать!)

        let texture = Texture::new("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/textures/wood.jpg".to_string()).unwrap();
        let ico_texture =
            Texture::new("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/textures/wall.jpg".to_string())
                .unwrap();
        let normal_map =
            Texture::new("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/normal_maps/wood_normals.jpg".to_string()).unwrap();
        let ico_normal_map =
            Texture::new("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/normal_maps/wall_normals.jpg".to_string()).unwrap();
        let mut data = loading_stl_model("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/stl_models/IcoSphere.stl").unwrap();
        loading_uv_obj_data(&mut data, "/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/uv_unwrap/Ico.obj");
        let mut mesh = PolygonMesh::new(
            data,
            vec![169, 169, 169],
            vec![0.9, 0.9, 0.9],
            vec![0.1, 0.1, 0.1],
            vec![0.0, 0.0, 0.0],
            50,
            Some(texture),
            Some(normal_map),
        );
        mesh.create_tbn();
        meshes.add(mesh);

        let mut data = loading_stl_model("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/stl_models/Table.stl").unwrap();
        loading_uv_obj_data(&mut data, "/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/uv_unwrap/Table.obj");
        let mut mesh = PolygonMesh::new(
            data,
            vec![169, 169, 169],
            vec![0.7, 0.7, 0.7],
            vec![0.3, 0.3, 0.3],
            vec![0.0, 0.0, 0.0],
            100,
            Some(ico_texture),
            Some(ico_normal_map),
        );
        mesh.create_tbn();
        meshes.add(mesh);

        let mut camera = FovCamera::default();
        camera.translate(&Vector3::new(7.35, -6.92, -4.95));
        camera.rotate(&Vector3::new(-43.0, 120.0, 0.0));

        let light = DistantLight::default();

        Controller {
            meshes,
            lights: vec![light],
            fov_camera: camera,
        }
    }
}

impl Controller {
    pub fn move_object(&mut self, translation: &Vector3<f32>, id: usize) {
        if id >= self.meshes.meshes.len() {
            return;
        }

        self.meshes.translate(translation, id);
    }

    pub fn rotate_object(&mut self, rotation: &Vector3<f32>, id: usize) {
        if id >= self.meshes.meshes.len() {
            return;
        }

        self.meshes.rotation(rotation, id);
    }

    pub fn scale_object(&mut self, scale: &Vector3<f32>, id: usize) {
        if id >= self.meshes.meshes.len() {
            return;
        }

        self.meshes.scale(scale, id);
    }
    
    pub fn move_camera(&mut self, translation: &Vector3<f32>) {
        self.fov_camera.translate(translation);
    }
    
    pub fn rotate_camera(&mut self, rotation: &Vector3<f32>) {
        self.fov_camera.rotate(rotation);
    }
    
    pub fn change_fov(&mut self, fov: f32) {
        self.fov_camera.fov = fov;
    }
    
    pub fn change_light_intensity(&mut self, light_intensity: f32) {
        self.lights[0].intensity = light_intensity;
    }
    
    pub fn change_light_back_intensity(&mut self, intensity: f32) {
        self.lights[0].back_intensity = intensity;
    }
    
    pub fn change_back_const(&mut self, ka: f32){
        self.lights[0].ka = ka;
    }
    
    pub fn move_light(&mut self, translation: &Vector3<f32>) {
        self.lights[0].translate(translation);
    }
    
    pub fn change_light_color(&mut self, color: &Vec<u8>) {
        self.lights[0].color = color.clone();
    }

    pub fn render(&mut self, image: &mut ColorImage) {
        Ray::render(&self.meshes, &self.fov_camera, &self.lights, image);
    }

    pub fn add_object(&mut self, object_path: Option<String>) -> Result<(), UiError> {
        if let Some(path) = object_path {
            let data = loading_stl_model(&path)?;
            let mesh = PolygonMesh::new(
                data,
                DEFAULT_COLOR.to_vec(),
                DEFAULT_KD.to_vec(),
                DEFAULT_KS.to_vec(),
                DEFAULT_KT.to_vec(),
                DEFAULT_LUMINOSITY,
                None,
                None,
            );
            self.meshes.add(mesh);
        } else {
            return Err(UiError::NoPathError);
        }

        Ok(())
    }

    pub fn add_properties(
        &mut self,
        id: usize,
        object_texture: String,
        object_normals: Option<String>,
        object_uv: String,
    ) -> Result<(), UiError> {
        if id >= self.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError)
        }
        
        let texture = Texture::new(object_texture);
        if let Err(_) = texture.clone() {
            return Err(UiError::LoadTextureError);
        }

        loading_uv_obj_data(&mut self.meshes.meshes[id].polygons, &object_uv)?;
        self.meshes.meshes[id].set_texture(Some(texture.unwrap()));
        
        if let Some(n) = object_normals {
            let normals_data = Texture::new(n);
            if let Err(_) = normals_data {
                return Err(UiError::LoadNormalsError);
            } else {
                self.meshes.meshes[id].set_normal_map(Some(normals_data.unwrap()));
                self.meshes.meshes[id].create_tbn();
            }
        }else{
            self.meshes.meshes[id].set_normal_map(None);
        }
        
        Ok(())
    }
}
