use crate::polygon::Polygon;
use crate::polygon::file_reader::Reader;
use crate::polygon::polygon_mesh::PolygonMesh;
use nalgebra::Point3;

pub struct MeshBuilder {
    pub polygons: Vec<Polygon>,
    pub color: Option<Vec<u8>>,
    pub kd: Option<Vec<f32>>,
    pub ks: Option<Vec<f32>>,
    pub kt: Option<Vec<f32>>,
    reader: Reader,
    part: usize,
}

impl MeshBuilder {
    pub fn new(reader: Reader) -> MeshBuilder {
        MeshBuilder {
            polygons: Vec::new(),
            color: None,
            kd: None,
            ks: None,
            kt: None,
            reader,
            part: 0,
        }
    }

    pub fn build_polygons(&mut self) {
        self.part += 1;
        let mut points: Vec<Point3<f32>> = Vec::new();

        while let Some(point) = self.reader.read_point() {
            points.push(point);

            if points.len() == 3 {
                self.polygons.push(Polygon::new(points.clone()));
                points.clear();
            }
        }
    }

    pub fn build_color(&mut self) {
        if let Some(color) = self.reader.read_color() {
            self.color = Some(color);
            self.part += 1;
        }
    }

    pub fn build_color_properties(&mut self) {
        if let Some(kd) = self.reader.read_color_props() {
            self.kd = Some(kd);
            self.part += 1;
        }

        if let Some(ks) = self.reader.read_color_props() {
            self.ks = Some(ks);
            self.part += 1;
        }

        if let Some(kt) = self.reader.read_color_props() {
            self.kt = Some(kt);
            self.part += 1;
        }
    }

    pub fn create(&mut self) -> Option<PolygonMesh> {
        if self.part != 5 {
            return None;
        }

        Some(PolygonMesh::new(
            self.polygons.clone(),
            self.color.clone()?,
            self.kd.clone()?,
            self.ks.clone()?,
            self.kt.clone()?,
            100,
            None,
            None,
        ))
    }
}
