mod light;
mod camera;
pub mod polygon;
mod ray_tracer;
pub mod texture;
mod ui;
mod controller;

use std::path::Path;
use std::time::Instant;
use::nalgebra::Vector3;
use egui::Color32;
use image::RgbaImage;
use nalgebra::Point3;
use crate::camera::FovCamera;
use crate::light::DistantLight;
use crate::polygon::file_reader::{loading_stl_model, loading_uv_obj_data, Reader};
use crate::polygon::Polygon;
use crate::polygon::polygon_mesh::polygon_meshes::PolygonMeshes;
use crate::polygon::polygon_mesh::PolygonMesh;
use crate::polygon::polygon_mesh_builder::MeshBuilder;
use crate::ray_tracer::Ray;
use crate::texture::Texture;
use crate::ui::App;

fn save_color_image(image: &egui::ColorImage, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let pixels: Vec<u8> = image
        .pixels
        .iter()
        .flat_map(|c| c.to_array())
        .collect();

    let image = RgbaImage::from_raw(
        image.width() as u32,
        image.height() as u32,
        pixels,
    ).ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;

    image.save(path)?;
    Ok(())
}

fn main() {
    // let mut reader = Reader::new("E://RustProjects//Sandbox//polygon.txt".to_string());
    // reader.open();
    // let mut builder = MeshBuilder::new(reader);
    // builder.build_polygons();
    // builder.build_color();
    // builder.build_color_properties();
    
    // meshes.add(builder.create().unwrap());

    // let mut reader = Reader::new("E://RustProjects//Sandbox//polygoninner.txt".to_string());
    // reader.open();
    // let mut builder = MeshBuilder::new(reader);
    // builder.build_polygons();
    // builder.build_color();
    // builder.build_color_properties();
    // meshes.add(builder.create().unwrap());
    //  
    // let mut reader = Reader::new("E://RustProjects//Sandbox//cube.txt".to_string());
    // reader.open();
    // let mut builder = MeshBuilder::new(reader);
    // builder.build_polygons();
    // builder.build_color();
    // builder.build_color_properties();
    //   
    // meshes.add(builder.create().unwrap());
    
    // let texture = Texture::new("E://RustProjects//Sandbox//wood.jpg".to_string()).unwrap();
    // let ico_texture = Texture::new("E://RustProjects//Sandbox//wall.jpg".to_string()).unwrap();
    // let normal_map = Texture::new("E://RustProjects//Sandbox/wood_normals.jpg".to_string()).unwrap();
    // let ico_normal_map = Texture::new("E://RustProjects//Sandbox/wall_normals.jpg".to_string()).unwrap();
    // 
    // let mut meshes = PolygonMeshes::default();
    // let mut data = loading_stl_model("E://RustProjects//Sandbox//IcoSphere.stl").unwrap();
    // loading_uv_obj_data(&mut data,"E://RustProjects//Sandbox//Ico.obj");
    // let mut mesh = PolygonMesh::new(data, vec![169, 169, 169], vec![0.9, 0.9, 0.9], vec![0.1, 0.1, 0.1], vec![0.0, 0.0, 0.0], 50, Some(texture), Some(normal_map));
    // mesh.create_tbn();
    // mesh.rotate(&Vector3::new(5.0, 0.0, 0.0));
    // mesh.translate(&Vector3::new(0.0, 0.0, 1.0));
    // mesh.scale(&Vector3::new(1.0, 1.0, 1.0));
    // meshes.add(mesh);
    // 
    // let mut data = loading_stl_model("E://RustProjects//Sandbox//Table.stl").unwrap();
    // loading_uv_obj_data(&mut data, "E://RustProjects//Sandbox//Table.obj");
    // let mut mesh = PolygonMesh::new(data, vec![169, 169, 169], vec![0.7, 0.7, 0.7], vec![0.3, 0.3, 0.3], vec![0.0, 0.0, 0.0], 100, Some(ico_texture), Some(ico_normal_map));
    // mesh.create_tbn();
    // meshes.add(mesh);
    // 
    // let light = DistantLight::default();
    // let mut camera = FovCamera::default();
    // camera.translate(&Vector3::new(7.35, -6.92, -4.95));
    // camera.yaw = 30.0;
    // camera.pitch = -43.0;
    // let mut image = egui::ColorImage::new([3820, 2160], vec![Color32::WHITE; 3820 * 2160]);
    // 
    // let start = Instant::now();
    // Ray::render(&meshes, &camera, &vec![light], &mut image);
    // let duration = start.elapsed();
    // println!("Time elapsed: {} ms", duration.as_millis());
    // 
    // save_color_image(&image, "output1.png");

    let options = eframe::NativeOptions{
        ..Default::default()
    };

    eframe::run_native(
        "Figures app",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles = [
                (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Body, egui::FontId::new(25.0, egui::FontFamily::Proportional)),
                (egui::TextStyle::Button, egui::FontId::new(25.0, egui::FontFamily::Proportional)),
            ].into();
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_visuals(egui::Visuals::light());

            Ok(Box::new(App::default()))
        }),
    ).expect("Ошибка при запуске");
}