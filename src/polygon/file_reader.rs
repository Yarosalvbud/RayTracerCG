use crate::polygon::Polygon;
use nalgebra::{Point2, Point3, Vector3};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::ui::errors::UiError;

pub struct Reader {
    file_name: String,
    reader: Option<BufReader<File>>,
}

impl Reader {
    pub fn new(file_name: String) -> Reader {
        Reader {
            file_name,
            reader: None,
        }
    }

    pub fn open(&mut self) -> Result<(), std::io::Error> {
        let file = match File::open(&self.file_name) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        self.reader = Some(BufReader::new(file));
        Ok(())
    }

    fn read_line(&mut self, buff: &mut String) -> Result<(), std::io::Error> {
        buff.clear();
        self.reader.as_mut().unwrap().read_line(buff)?;
        *buff = buff.trim().to_string();

        Ok(())
    }

    fn parse_point_from_str(&self, buff: &str) -> Option<Point3<f32>> {
        let parts = buff.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 3 {
            return None;
        }

        let x = parts[0].parse::<f32>();
        let y = parts[1].parse::<f32>();
        let z = parts[2].parse::<f32>();

        if let (Ok(x), Ok(y), Ok(z)) = (x, y, z) {
            Some(Point3::new(x, y, z))
        } else {
            None
        }
    }

    fn parse_color_from_str(&self, buff: &str) -> Option<Vec<u8>> {
        let parts = buff.split_whitespace().collect::<Vec<&str>>();

        if parts.len() != 3 {
            return None;
        }

        let x = parts[0].parse::<u8>();
        let y = parts[1].parse::<u8>();
        let z = parts[2].parse::<u8>();

        if let (Ok(x), Ok(y), Ok(z)) = (x, y, z) {
            Some(vec![x, y, z])
        } else {
            None
        }
    }
    pub fn read_point(&mut self) -> Option<Point3<f32>> {
        if self.reader.is_none() {
            return None;
        }

        let mut buff = String::new();
        let read_result = self.read_line(&mut buff);
        if let Err(_) = read_result {
            return None;
        }
        if buff == "Polygon Start" {
            let read_result = self.read_line(&mut buff);
            if let Err(_) = read_result {
                return None;
            }
        }

        self.parse_point_from_str(&buff)
    }

    pub fn read_color(&mut self) -> Option<Vec<u8>> {
        if self.reader.is_none() {
            return None;
        }

        let mut buff = String::new();
        let read_result = self.read_line(&mut buff);
        if let Err(_) = read_result {
            None
        } else {
            self.parse_color_from_str(&buff)
        }
    }

    pub fn read_color_props(&mut self) -> Option<Vec<f32>> {
        if self.reader.is_none() {
            return None;
        }

        let mut buff = String::new();
        let read_result = self.read_line(&mut buff);
        if let Err(_) = read_result {
            None
        } else {
            let result = self.parse_point_from_str(&buff);
            if let Some(point) = result {
                return Some(vec![point.x, point.y, point.z]);
            }
            None
        }
    }
}

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