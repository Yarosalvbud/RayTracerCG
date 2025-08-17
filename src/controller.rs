use crate::camera::FovCamera;
use crate::light::DistantLight;
use crate::polygon::polygon_mesh::polygon_meshes::PolygonMeshes;
use crate::polygon::polygon_mesh_builder::PolygonMeshBuilder;
use crate::ray_tracer::Ray;
use crate::ui::errors::UiError;
use eframe::epaint::Color32;
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
        let mut meshes = PolygonMeshes::default();
        let mut builder = PolygonMeshBuilder::new();

        builder.build_polygons(Some("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/stl_models/IcoSphere.stl".to_string())).expect("Ошибка при загрузке стартовой сцены");
        let mesh = builder.build_object(
            DEFAULT_COLOR.to_vec(),
            DEFAULT_KD.to_vec(),
            DEFAULT_KS.to_vec(),
            DEFAULT_KT.to_vec(),
            DEFAULT_LUMINOSITY,
        );

        meshes.add(mesh);
        meshes
            .load_uv(
                0,
                "/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/uv_unwrap/Ico.obj",
            )
            .expect("Ошибка при загрузке стартовой сцены");
        meshes.set_texture(0, Some("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/textures/wood.jpg".to_string())).expect("Ошибка при загрузке стартовой сцены");
        meshes.set_normal_map(0, Some("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/normal_maps/wood_normals.jpg".to_string())).expect("Ошибка при загрузке стартовой сцены");

        builder.build_polygons(Some("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/stl_models/Table.stl".to_string())).expect("Ошибка при загрузке стартовой сцены");
        let mesh = builder.build_object(
            DEFAULT_COLOR.to_vec(),
            DEFAULT_KD.to_vec(),
            DEFAULT_KS.to_vec(),
            DEFAULT_KT.to_vec(),
            DEFAULT_LUMINOSITY,
        );

        meshes.add(mesh);
        meshes.load_uv(1, "/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/uv_unwrap/Table.obj").expect("Ошибка при загрузке стартовой сцены");
        meshes.set_texture(1, Some("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/textures/wall.jpg".to_string())).expect("Ошибка при загрузке стартовой сцены");
        meshes.set_normal_map(1, Some("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/normal_maps/wall_normals.jpg".to_string())).expect("Ошибка при загрузке стартовой сцены");

        let mut camera = FovCamera::default();
        camera.translate(&Vector3::new(7.35, -6.92, -4.95));
        camera.rotate(&Vector3::new(-43.0, 120.0, 0.0));

        let mut light = DistantLight::default();
        light.translate(&Vector3::new(4.07, -1.0, 5.9));

        Controller {
            meshes,
            lights: vec![light],
            fov_camera: camera,
        }
    }
}

impl Controller {
    pub fn move_object(&mut self, translation: &Vector3<f32>, id: usize) -> Result<(), UiError> {
        if id >= self.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.meshes.translate(translation, id);
        Ok(())
    }

    pub fn rotate_object(&mut self, rotation: &Vector3<f32>, id: usize) -> Result<(), UiError> {
        if id >= self.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.meshes.rotation(rotation, id);
        Ok(())
    }

    pub fn scale_object(&mut self, scale: &Vector3<f32>, id: usize) -> Result<(), UiError> {
        if id >= self.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.meshes.scale(scale, id);
        Ok(())
    }

    pub fn remove_object(&mut self, id: usize) -> Result<(), UiError> {
        if id >= self.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.meshes.remove(id);
        Ok(())
    }

    pub fn move_camera(&mut self, translation: &Vector3<f32>) {
        self.fov_camera.translate(translation);
    }

    pub fn rotate_camera(&mut self, rotation: &Vector3<f32>) {
        self.fov_camera.rotate(rotation);
    }

    pub fn change_fov(&mut self, fov: f32) {
        self.fov_camera.change_fov(fov);
    }

    pub fn change_light_intensity(&mut self, light_intensity: f32) {
        self.lights[0].change_light_intensity(light_intensity);
    }

    pub fn change_light_back_intensity(&mut self, intensity: f32) {
        self.lights[0].change_light_back_intensity(intensity);
    }
    
    pub fn change_object_luminosity(&mut self, id: usize, luminosity: i32)->Result<(),UiError> {
        if id >= self.meshes.meshes.len() {
            return Err(UiError::ObjectNotFoundError);
        }
        
        self.meshes.set_luminosity(id, luminosity);
        
        Ok(())
    }

    pub fn change_back_const(&mut self, ka: f32) {
        self.lights[0].change_ka(ka);
    }

    pub fn move_light(&mut self, translation: &Vector3<f32>) {
        self.lights[0].translate(translation);
    }

    pub fn change_light_color(&mut self, color: &Vec<u8>) {
        self.lights[0].change_color(color.clone());
    }

    pub fn render(&mut self, image: &mut ColorImage, bg_color: Color32) {
        let bg_color = vec![bg_color.r(), bg_color.g(), bg_color.b()];
        Ray::render(
            &self.meshes,
            &self.fov_camera,
            &self.lights,
            image,
            bg_color,
        );
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

        self.meshes.set_kd(id, kd.to_vec());
        self.meshes.set_ks(id, ks.to_vec());
        self.meshes.set_kt(id, kt.to_vec());
        self.meshes
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
        self.meshes.add(mesh);

        Ok(())
    }

    pub fn add_properties(
        &mut self,
        id: usize,
        object_texture: Option<String>,
        object_normals: Option<String>,
        object_uv: String,
    ) -> Result<(), UiError> {
        if id >= self.meshes.length() {
            return Err(UiError::ObjectNotFoundError);
        }

        self.meshes.load_uv(id, &object_uv)?;
        self.meshes.set_texture(id, object_texture)?;
        self.meshes.set_normal_map(id, object_normals)?;

        Ok(())
    }
}
