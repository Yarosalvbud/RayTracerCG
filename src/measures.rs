use std::fs::File;
use std::io::Write;
use std::time::Instant;
use eframe::epaint::{Color32, ColorImage};
use nalgebra::Vector3;
use crate::polygon::polygon_mesh::polygon_meshes::PolygonMeshes;
use crate::ray_tracer::Ray;
use crate::scene::Scene;

const NUM_REPS: i32 = 30;
const MIN_DEPTH: i32 = 1;
const MAX_DEPTH: i32 = 11;

const START_SIZE: usize = 512;
const SIZE_SCALE: usize = 2;
const NUM_IMAGES: i32 = 3;
const MAX_MESHES: usize = 7;

const DEPTH: usize = 3;

pub fn measure_time(path: &str) {
    let (mut scene, _) = Scene::loading_scene(path).unwrap();
    for camera in scene.cameras.cameras.iter_mut() {
        camera.rotate(&Vector3::zeros());
    }
    let mut start_size: usize = START_SIZE;
    for _ in 0..NUM_IMAGES {
        let mut image = ColorImage::new([start_size, start_size], vec![Color32::WHITE; start_size * start_size]);

        println!("{start_size}");
        for depth in MIN_DEPTH..MAX_DEPTH {
            let mut all_time = 0;
            let filename = format!("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/measures/tracing_{}_{}.txt", start_size, depth);
            let mut file = File::create(&filename).expect("Не удалось создать файл");

            for _ in 0..NUM_REPS {
                let bg_color = vec![0u8, 0u8, 0u8];
                let start = Instant::now();
                Ray::render(&scene.meshes, &scene.cameras.active_camera(), &scene.lights, &mut image, bg_color, depth);
                let end = start.elapsed().as_millis();

                all_time += end;
                writeln!(file, "{}", end).unwrap();
            }

            println!("{:?}", all_time as f64 / NUM_REPS as f64);
        }

        start_size *= SIZE_SCALE;
    }
}

pub fn measure_time_polygons(path: &str) {
    let (mut scene, _) = Scene::loading_scene(path).unwrap();
    for camera in scene.cameras.cameras.iter_mut() {
        camera.rotate(&Vector3::zeros());
    }

    let meshes = scene.meshes.clone();
    let mut image = ColorImage::new([1920, 1080], vec![Color32::WHITE; 1920 * 1080]);
    let mut counter = 20;

    for i in 1..MAX_MESHES {
        let mut meshes_free = Vec::new();
        let mut meshes_full = Vec::new();
        for j in 0..i {
            let mut mesh = meshes.meshes[j].clone();
            mesh.set_texture(None);
            mesh.set_normal_map(None);

            meshes_free.push(mesh);

            let mesh_full = meshes.meshes[j].clone();
            meshes_full.push(mesh_full);
        }

        let mut all_time_full = 0;
        let mut all_time_empty = 0;
        let filename_full = format!("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/measures_meshes/full_{}.txt", counter);
        let filename_empty = format!("/Users/aroslavbudancev/Documents/Projects/RayTracerCG/src/data/measures_meshes/empty_{}.txt", counter);
        let mut file_full = File::create(&filename_full).expect("Не удалось создать файл");
        let mut file_empty = File::create(&filename_empty).expect("Не удалось создать файл");

        for _ in 0..NUM_REPS {
            let meshes_full_structure = PolygonMeshes::new(meshes_full.clone());
            let meshes_empty_structure = PolygonMeshes::new(meshes_free.clone());

            let bg_color = vec![0u8, 0u8, 0u8];
            let start = Instant::now();
            Ray::render(&meshes_full_structure, &scene.cameras.active_camera(), &scene.lights, &mut image, bg_color.clone(), DEPTH as i32);
            let end = start.elapsed().as_millis();

            all_time_full += end;
            writeln!(file_full, "{}", end).unwrap();

            let start = Instant::now();
            Ray::render(&meshes_empty_structure, &scene.cameras.active_camera(), &scene.lights, &mut image, bg_color.clone(), DEPTH as i32);
            let end = start.elapsed().as_millis();

            all_time_empty += end;
            writeln!(file_empty, "{}", end).unwrap();
        }

        println!("Full: {:?}", all_time_full as f64 / NUM_REPS as f64);
        println!("Empty: {:?}", all_time_empty as f64 / NUM_REPS as f64);
        counter += 20;
    }
}