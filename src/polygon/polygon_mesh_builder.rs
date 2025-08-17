use crate::polygon::Polygon;
use crate::polygon::polygon_mesh::PolygonMesh;
use crate::polygon::file_reader::loading_stl_model;
use crate::texture::Texture;
use crate::ui::errors::UiError;

pub struct PolygonMeshBuilder {
    data: Vec<Polygon>,
}


impl PolygonMeshBuilder {
    pub fn new() -> PolygonMeshBuilder {
        PolygonMeshBuilder{
            data: Vec::new(),
        }
    }

    pub fn build_polygons(&mut self, object_path: Option<String>)-> Result<(), UiError>{
        if let Some(object_path) = object_path {
            let data = loading_stl_model(&object_path)?;
            self.data = data;
        }else {
            return Err(UiError::NoPathError);
        }

        Ok(())
    }

    pub fn build_object(&mut self, color: Vec<u8>, kd: Vec<f32>, ks: Vec<f32>, kt: Vec<f32>, luminosity: i32)->PolygonMesh{
        PolygonMesh::new(
            self.data.clone(),
            color,
            kd,
            ks,
            kt,
            luminosity,
            None,
            None
        )
    }
    
    pub fn build_texture(object: &mut PolygonMesh, texture: Option<String>) -> Result<(), UiError>{
        if let Some(texture) = texture {
            let texture = Texture::new(texture);
            if let Err(_) = texture.clone() {
                return Err(UiError::LoadTextureError);
            } else {
                object.set_texture(Some(texture.unwrap()));
            }
        } else {
            object.set_texture(None);
        }
        
        Ok(())
    }
    
    pub fn build_normal_map(object: &mut PolygonMesh, normals: Option<String>) -> Result<(), UiError>{
        if let Some(n) = normals {
            let normals_data = Texture::new(n);
            if let Err(_) = normals_data {
                return Err(UiError::LoadNormalsError);
            } else {
                object.set_normal_map(Some(normals_data.unwrap()));
                object.create_tbn();
            }
        } else {
            object.set_normal_map(None);
        }
        
        Ok(())
    }
    
    
}