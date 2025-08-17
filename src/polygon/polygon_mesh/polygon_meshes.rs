use nalgebra::Vector3;
use crate::polygon::polygon_mesh::PolygonMesh;
use crate::polygon::polygon_mesh_builder::PolygonMeshBuilder;
use crate::ui::errors::UiError;

#[derive(Clone, Debug)]
pub struct PolygonMeshes{
    pub meshes: Vec<PolygonMesh>,
}

impl Default for PolygonMeshes{
    fn default() -> PolygonMeshes {
        PolygonMeshes{
            meshes: Vec::new(),
        }
    }
}

impl PolygonMeshes{
    pub fn add(&mut self, polygon_mesh: PolygonMesh){
        self.meshes.push(polygon_mesh);
    }
    
    pub fn remove(&mut self, id: usize){
        self.meshes.remove(id);
    }
    
    pub fn translate(&mut self, translation: &Vector3<f32>, id: usize){
        self.meshes[id].translate(translation);
    }
    
    pub fn rotation(&mut self, rotation: &Vector3<f32>, id: usize){
        self.meshes[id].rotate(rotation);
    }
    
    pub fn scale(&mut self, scale: &Vector3<f32>, id: usize){
        self.meshes[id].scale(scale);
    }
    
    pub fn new(objects: Vec<PolygonMesh>) -> PolygonMeshes{
        PolygonMeshes{
            meshes: objects,
        }
    }
    
    pub fn get_object(&self, id: usize)->PolygonMesh{
        self.meshes[id].clone()
    }
    
    pub fn transform_to_world(&self)->PolygonMeshes{
        let mut objects: Vec<PolygonMesh> = Vec::new();
        
        for object in self.meshes.iter() {
            objects.push(object.transform_to_world());
        }
        
        PolygonMeshes::new(objects)
    }
    
    pub fn length(&self)->usize{
        self.meshes.len()
    }
    
    pub fn set_texture(&mut self, id: usize, texture: Option<String>)->Result<(), UiError>{
        PolygonMeshBuilder::build_texture(&mut self.meshes[id], texture)
    }
    
    pub fn set_normal_map(&mut self, id: usize, normals: Option<String>)->Result<(), UiError>{
        PolygonMeshBuilder::build_normal_map(&mut self.meshes[id], normals)
    }
    
    pub fn create_tbn(&mut self, id: usize){
        self.meshes[id].create_tbn();
    }
    
    pub fn set_kd(&mut self, id: usize, kd: Vec<f32>){
        self.meshes[id].set_kd(kd);
    }
    
    pub fn set_kt(&mut self, id: usize, kt: Vec<f32>){
        self.meshes[id].set_kt(kt);
    }
    
    pub fn set_ks(&mut self, id: usize, ks: Vec<f32>){
        self.meshes[id].set_ks(ks);
    }
    
    pub fn set_color(&mut self, id: usize, color: Vec<u8>){
        self.meshes[id].set_color(color);
    }

    pub fn remove_objects(&mut self, id: usize){
        self.meshes.remove(id);
    }

    pub fn load_uv(&mut self, id: usize, uv: &str)->Result<(), UiError>{
        self.meshes[id].load_uv(uv)
    }
}