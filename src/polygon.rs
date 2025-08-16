pub mod file_reader;
pub mod polygon_mesh;
pub mod polygon_mesh_builder;

use crate::ray_tracer::Ray;
use crate::texture::Texture;
use egui::Color32;
use nalgebra::{Matrix3, Matrix4, Point2, Point3, RowVector3, Vector3};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertexes: Vec<Point3<f32>>,
    normal: Vector3<f32>,
    pub uv: Vec<Point2<f32>>,
    pub tbn: Arc<Matrix3<f32>>,
}

#[derive(Debug, Clone)]
pub struct RayIntersect {
    pub intersection_point: Point3<f32>,
    pub barycentric_coords: Point3<f32>,
    pub t: f32,
    pub surface_normal: Vector3<f32>,
    pub texture_color: Option<Color32>,
}

impl RayIntersect {
    pub fn new(
        intersection_point: Point3<f32>,
        barycentric_coords: Point3<f32>,
        t: f32,
        surface_normal: Vector3<f32>,
        texture_color: Option<Color32>,
    ) -> RayIntersect {
        RayIntersect {
            intersection_point,
            barycentric_coords,
            t,
            surface_normal,
            texture_color,
        }
    }
}

impl Polygon {
    pub fn new(vertexes: Vec<Point3<f32>>) -> Polygon {
        let u = vertexes[2] - vertexes[1];
        let v = vertexes[0] - vertexes[2];

        let normal = u.cross(&v) / 2.0;

        Polygon {
            vertexes,
            normal,
            uv: Vec::new(),
            tbn: Arc::new(Matrix3::identity()),
        }
    }
    
    pub fn move_to_center(&mut self, point: &Point3<f32>) {
        for vertex in self.vertexes.iter_mut() {
            vertex.x -= point.x;
            vertex.y -= point.y;
            vertex.z -= point.z;
        }
    }

    pub fn new_with_normals(vertexes: Vec<Point3<f32>>, normal: Vector3<f32>) -> Polygon {
        let u = vertexes[2] - vertexes[1];
        let v = vertexes[0] - vertexes[2];

        let square = u.cross(&v).norm() / 2.0;
        let normal_full = square * normal;

        Polygon {
            vertexes,
            normal: normal_full,
            uv: Vec::new(),
            tbn: Arc::new(Matrix3::identity()),
        }
    }
    
    pub fn transform(&mut self, transform: &Matrix4<f32>){
        for vertex in self.vertexes.iter_mut() {
            *vertex = Point3::from((transform * vertex.to_homogeneous()).xyz());
        }
        let u = self.vertexes[2] - self.vertexes[1];
        let v = self.vertexes[0] - self.vertexes[2];

        self.normal = u.cross(&v) / 2.0;
        
        let rotation = transform.fixed_view::<3, 3>(0, 0);
        self.tbn = Arc::from(rotation * &*self.tbn);
    }

    pub fn color_from_texture(&self, texture: &Texture, point: &Point2<f32>) -> Color32 {
        texture.sample(point.x, point.y)
    }

    pub fn create_tbn(&mut self) {
        let normal = self.normal.normalize().transpose();
        let edge_first = self.vertexes[1] - self.vertexes[0];
        let edge_second = self.vertexes[2] - self.vertexes[0];

        let delta_uv_first = self.uv[1] - self.uv[0];
        let delta_uv_second = self.uv[2] - self.uv[0];

        let f = 1.0 / (delta_uv_first.x * delta_uv_second.y - delta_uv_second.x * delta_uv_first.y);

        let mut tangent = RowVector3::new(
            f * (delta_uv_second.y * edge_first.x - delta_uv_first.y * edge_second.x),
            f * (delta_uv_second.y * edge_first.y - delta_uv_first.y * edge_second.y),
            f * (delta_uv_second.y * edge_first.z - delta_uv_first.y * edge_second.z),
        )
        .normalize();
        tangent = (tangent - tangent.dot(&normal) * normal).normalize();

        let bitangent = normal.cross(&tangent).normalize();

        self.tbn = Arc::new(Matrix3::from_rows(&[tangent, bitangent, normal]).transpose());
    }

    pub fn normal_mapping(&self, texture: &Texture, point: &Point2<f32>) -> Vector3<f32> {
        let normal = texture.sample(point.x, point.y);
        let poly_normal = Vector3::new(
            (normal[0] as f32 / 255.0) * 2.0 - 1.0,
            (normal[1] as f32 / 255.0) * 2.0 - 1.0,
            (normal[2] as f32 / 255.0) * 2.0 - 1.0,
        );
        
        (self.tbn.as_ref() * poly_normal).normalize()
    }

    pub fn ray_intersect(
        &self,
        ray: &Ray,
        texture: &Option<Texture>,
        normal_map: &Option<Texture>,
    ) -> Option<RayIntersect> {
        let n_dot_dir = self.normal.dot(&ray.direction);
        let denom = self.normal.dot(&self.normal);

        if n_dot_dir.abs() < f32::EPSILON {
            return None;
        }

        let d = -self.normal.dot(&self.vertexes[0].coords);

        let t = -(self.normal.dot(&ray.origin.coords) + d) / n_dot_dir;
        if t < 0.0 {
            return None;
        }

        let intersection_point = ray.origin.coords + t * ray.direction;

        let mut sub_area = (self.vertexes[2] - self.vertexes[1])
            .cross(&(intersection_point - self.vertexes[1].coords));

        let mut u = self.normal.dot(&sub_area);
        if u < 0.0 {
            return None;
        }

        sub_area = (self.vertexes[0] - self.vertexes[2])
            .cross(&(intersection_point - self.vertexes[2].coords));

        let mut v = self.normal.dot(&sub_area);
        if v < 0.0 {
            return None;
        }

        sub_area = (self.vertexes[1] - self.vertexes[0])
            .cross(&(intersection_point - self.vertexes[0].coords));
        let mut w = self.normal.dot(&sub_area);
        if w < 0.0 {
            return None;
        }

        u /= 2.0 * denom;
        v /= 2.0 * denom;
        w /= 2.0 * denom;

        if let Some(texture) = texture {
            let point = Point2::new(
                self.uv[0].x * u + self.uv[1].x * v + self.uv[2].x * w,
                self.uv[0].y * u + self.uv[1].y * v + self.uv[2].y * w,
            );

            if let Some(normal_map) = normal_map {
                return Some(RayIntersect::new(
                    <Point3<f32>>::from(intersection_point),
                    Point3::new(u, v, w),
                    t,
                    self.normal_mapping(&normal_map, &point),
                    Some(self.color_from_texture(&texture, &point)),
                ));
            }
            return Some(RayIntersect::new(
                <Point3<f32>>::from(intersection_point),
                Point3::new(u, v, w),
                t,
                self.normal.normalize(),
                Some(self.color_from_texture(&texture, &point)),
            ));
        }

        Some(RayIntersect::new(
            <Point3<f32>>::from(intersection_point),
            Point3::new(u, v, w),
            t,
            self.normal.normalize(),
            None,
        ))
    }
}
