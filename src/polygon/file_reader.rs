use crate::polygon::Polygon;
use nalgebra::{Point2, Point3, Vector3};
use crate::ui::errors::UiError;

pub fn loading_stl_model(file_name: &str) -> Result<Vec<Polygon>, UiError> {
    use std::fs::OpenOptions;
    let file = OpenOptions::new().read(true).open(file_name);
    if let Err(_) = file {
        return Err(UiError::BadStlError);
    }
    
    let stl = stl_io::read_stl(&mut file.unwrap());
    
    if let Err(_) = stl {
        return Err(UiError::BadStlError);
    }
    let stl = stl.unwrap();
    
    let vertices: Vec<Point3<f32>> = stl
        .vertices
        .iter()
        .map(|v| Point3::new(v[0], v[1], v[2]))
        .collect();

    Ok(stl.faces
        .iter()
        .map(|poly| {
            Polygon::new_with_normals(
                poly.vertices.iter().map(|idx| vertices[*idx]).collect(),
                Vector3::new(poly.normal[0], poly.normal[1], poly.normal[2]),
            )
        })
        .collect())
}

pub fn loading_uv_obj_data(polygons: &mut Vec<Polygon>, file_name: &str)->Result<(), UiError>{
    let input = tobj::load_obj(file_name, &tobj::LoadOptions::default());
    if let Err(_) =  input {
        return Err(UiError::BadUvError);
    }
    
    let (models, _) = input.unwrap();
    let mesh = &models[0].mesh;

    let tex_coords = &mesh.texcoords;
    let tex_coords_indices = &mesh.texcoord_indices;
        
    let mut to_polygons: Vec<Point2<f32>> = Vec::new();
    let mut polygons_uv: Vec<Vec<Point2<f32>>> = Vec::new();
    let mut to_polygons_idx = 0;
        
    for idx in tex_coords_indices.iter(){
        to_polygons.push(Point2::new(tex_coords[2 * *idx as usize], tex_coords[2 * *idx as usize + 1]));
            
        if to_polygons.len() == 3{
            if to_polygons_idx >= polygons.len() {
                return Err(UiError::BadUVError);
            }
                
            polygons_uv.push(to_polygons.clone());
            to_polygons_idx += 1;
            to_polygons.clear();
        }
    }

    if polygons_uv.len() != polygons.len(){
        return Err(UiError::BadUVError);
    }else {
        polygons.iter_mut().zip(polygons_uv).for_each(|(poly, uv)| poly.uv = uv);
    }
    
    Ok(())
}