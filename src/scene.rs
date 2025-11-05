#[allow(dead_code)]
use crate::camera::FovCamera;
use crate::light::DistantLight;
use crate::light::lights::Lights;
use crate::polygon::polygon_mesh::polygon_meshes::PolygonMeshes;
use crate::polygon::polygon_mesh_builder::PolygonMeshBuilder;
use crate::ui::errors::UiError;
use nalgebra::{Point3, Vector3};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use crate::camera::cameras::Cameras;

#[derive(Debug, Deserialize)]
struct Object {
    path: String,
    texture: Option<String>,
    normal_map: Option<String>,
    kd: [f32; 3],
    ks: [f32; 3],
    kt: [f32; 3],
    color: [u8; 3],
    luminosity: i32,
    translation: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
}

#[derive(Debug, Deserialize)]
struct Camera {
    fov: f32,
    yaw: f32,
    pitch: f32,
    origin: [f32; 3],
    target: [f32; 3],
}

#[derive(Debug, Deserialize)]
struct Light {
    position: [f32; 3],
    color: [u8; 3],
    intensity: f32,
}

#[derive(Debug, Deserialize)]
struct LightProps {
    back_color: [u8; 3],
    back_intensity: f32,
    back_const: f32,
}

#[derive(Debug, Deserialize)]
struct LightWrapper {
    light: Light,
}

#[derive(Debug, Deserialize)]
struct ObjectWrapper {
    object: Object,
}

#[derive(Debug, Deserialize)]
struct CameraWrapper {
    camera: Camera,
}

#[derive(Debug, Deserialize)]
struct SceneYaml {
    objects: Vec<ObjectWrapper>,
    cameras: Vec<CameraWrapper>,
    lights: Vec<LightWrapper>,
    lights_props: LightProps,
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub meshes: PolygonMeshes,
    pub cameras: Cameras,
    pub lights: Lights,
}

impl Default for Scene {
    fn default() -> Self {
        Scene{
            meshes: PolygonMeshes::default(),
            cameras: Cameras::default(),
            lights: Lights::default(),
        }
    }
}

impl Scene {
    pub fn new(camera: Cameras, objects: PolygonMeshes, lights: Lights) -> Self {
        Scene {
            meshes: objects,
            cameras: camera,
            lights,
        }
    }

    fn parse_lights(scene: &SceneYaml) -> Result<Lights, UiError> {
        let mut lights = Vec::new();

        for light in &scene.lights {
            if light.light.intensity > 1.0 {
                return Err(UiError::LightIntensityError);
            } else {
                lights.push(DistantLight::new(
                    Point3::from(light.light.position),
                    light.light.intensity,
                    Vec::from(light.light.color),
                ));
            }
        }

        if scene.lights_props.back_intensity > 1.0 {
            return Err(UiError::LightIntensityError);
        }

        if scene.lights_props.back_const > 1.0 {
            return Err(UiError::BackConstError);
        }

        let scene_lights = Lights::new(
            scene.lights_props.back_intensity,
            scene.lights_props.back_const,
            Vec::from(scene.lights_props.back_color),
            lights,
        );
        Ok(scene_lights)
    }

    fn parse_camera(scene: &SceneYaml) -> Result<Cameras, UiError> {
        let mut cameras = Vec::new();

        for camera in &scene.cameras {
            if camera.camera.fov > 180.0{
                return Err(UiError::BadFovError);
            }

            let camera = FovCamera::new(
                Point3::from(camera.camera.origin),
                Point3::from(camera.camera.target),
                camera.camera.yaw,
                camera.camera.pitch,
                camera.camera.fov,
            );

            cameras.push(camera);
        }

        Ok(Cameras::new(cameras))
    }

    fn parse_objects(scene: &SceneYaml) -> Result<(PolygonMeshes, Vec<String>), UiError> {
        let mut objects = Vec::new();
        let mut names = Vec::new();

        for object in scene.objects.iter() {
            let mut builder = PolygonMeshBuilder::new();
            builder.build_polygons(Some(object.object.path.clone()))?;

            let name = Path::new(&object.object.path).file_name();
            if let Some(name) = name {
                names.push(name.to_str().unwrap().to_string());
            }

            let kd = Vec::from(object.object.kd);
            let ks = Vec::from(object.object.ks);
            let kt = Vec::from(object.object.kt);

            if object.object.luminosity < 0 {
                return Err(UiError::LuminosityError);
            }

            if kd[0] < 0.0
                || kd[1] < 0.0
                || kd[2] < 0.0
                || ks[0] < 0.0
                || ks[1] < 0.0
                || ks[2] < 0.0
                || kt[0] < 0.0
                || kt[1] < 0.0
                || kt[2] < 0.0
            {
                return Err(UiError::ColorPropsSignError);
            }

            if kd[0] + kt[0] + ks[0] > 1.0
                || kd[1] + ks[1] + kt[1] > 1.0
                || kd[2] + ks[2] + kt[2] > 1.0
            {
                return Err(UiError::ColorPropertiesError);
            }

            let mut scene_object = builder.build_object(
                Vec::from(object.object.color),
                kd,
                ks,
                kt,
                object.object.luminosity,
            );
            PolygonMeshBuilder::build_texture(&mut scene_object, object.object.texture.clone())?;
            PolygonMeshBuilder::build_normal_map(&mut scene_object, object.object.normal_map.clone())?;
            scene_object.translate(&Vector3::from(object.object.translation));
            scene_object.rotate(&Vector3::from(object.object.rotation));
            scene_object.scale(&Vector3::from(object.object.scale));
            objects.push(scene_object);
        }

        Ok((PolygonMeshes::new(objects), names))
    }

    pub fn loading_scene(file_name: &str) -> Result<(Scene, Vec<String>), Box<dyn std::error::Error>> {
        let yaml_str = fs::read_to_string(file_name)?;
        let scene_yaml: SceneYaml = serde_yaml::from_str(&yaml_str)?;
        let cameras = Self::parse_camera(&scene_yaml)?;
        let lights = Self::parse_lights(&scene_yaml)?;
        let (objects, names) = Self::parse_objects(&scene_yaml)?;

        Ok((Scene::new(cameras, objects, lights), names))
    }
}
