use nalgebra::Vector3;
use crate::polygon::polygon_mesh::PolygonMesh;


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
}