pub mod polygon_meshes;

use crate::polygon::{Polygon, RayIntersect};
use crate::ray_tracer::Ray;
use crate::texture::Texture;
use nalgebra::{Matrix4, Point3, Vector3};

#[derive(Debug, Clone)]
struct Sphere {
    radius: f32,
    center: Point3<f32>,
}

#[derive(Clone, Debug)]
pub struct PolygonMesh {
    pub polygons: Vec<Polygon>,
    pub color: Vec<u8>,
    pub kd: Vec<f32>,
    pub ks: Vec<f32>,
    pub kt: Vec<f32>,
    pub luminosity: i32,
    texture: Option<Texture>,
    normal_map: Option<Texture>,
    enclosing_sphere: Sphere,
    rotation: Matrix4<f32>,
    translation: Matrix4<f32>,
    scale: Matrix4<f32>,
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32) -> Sphere {
        Sphere { radius, center }
    }
}

impl PolygonMesh {
    pub fn new(
        polygons: Vec<Polygon>,
        color: Vec<u8>,
        kd: Vec<f32>,
        ks: Vec<f32>,
        kt: Vec<f32>,
        luminosity: i32,
        texture: Option<Texture>,
        normal_map: Option<Texture>,
    ) -> PolygonMesh {
        let center = Self::barycenter(&polygons);
        let radius = Self::sphere_radius(&polygons, &center);

        let sphere = Sphere::new(center, radius);

        PolygonMesh {
            polygons,
            color,
            kd,
            ks,
            kt,
            luminosity,
            texture,
            normal_map,
            enclosing_sphere: sphere,
            rotation: Matrix4::identity(),
            translation: Matrix4::identity(),
            scale: Matrix4::identity(),
        }
    }

    fn barycenter(polygons: &Vec<Polygon>) -> Point3<f32> {
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut z_sum = 0.0;
        let mut count = 0;

        for polygon in polygons {
            for point in polygon.vertexes.iter() {
                x_sum += point.x;
                y_sum += point.y;
                z_sum += point.z;
                count += 1;
            }
        }

        Point3::new(
            x_sum / count as f32,
            y_sum / count as f32,
            z_sum / count as f32,
        )
    }

    fn sphere_radius(polygons: &Vec<Polygon>, center: &Point3<f32>) -> f32 {
        let mut max_radius = 0.0;

        for polygon in polygons {
            for point in polygon.vertexes.iter() {
                let distance = nalgebra::distance(center, point);

                if distance > max_radius {
                    max_radius = distance;
                }
            }
        }

        max_radius
    }

    pub fn sphere_intersect(&self, ray: &Ray) -> bool {
        let to_sphere = self.enclosing_sphere.center - ray.origin;
        let k1 = ray.direction.dot(&ray.direction);
        let k2 = 2.0 * to_sphere.dot(&ray.direction);
        let k3 =
            to_sphere.dot(&to_sphere) - self.enclosing_sphere.radius * self.enclosing_sphere.radius;

        let d = k2 * k2 - 4.0 * k1 * k3;
        if d < 0.0 {
            return false;
        }

        true
    }
    
    pub fn set_normal_map(&mut self, normal_map: Option<Texture>) {
        self.normal_map = normal_map;
    }
    
    pub fn set_texture(&mut self, texture: Option<Texture>) {
        self.texture = texture;
    }

    pub fn intersect(&self, ray: &Ray) -> Option<RayIntersect> {
        let mut intersection: Option<RayIntersect> = None;
        let mut t_min = f32::INFINITY;

        for polygon in self.polygons.iter() {
            let hit = polygon.ray_intersect(ray, &self.texture, &self.normal_map);
            let hit_p = hit.clone();

            if let Some(hit) = hit {
                if hit.t < t_min {
                    t_min = hit.t;
                    intersection = hit_p;
                }
            }
        }

        intersection
    }

    pub fn obj_to_world_mtr(&self) -> Matrix4<f32> {
        self.translation * self.rotation * self.scale
    }

    pub fn translate(&mut self, translation: &Vector3<f32>) {
        let translation = Matrix4::new_translation(translation);
        self.translation = translation * self.translation;
    }

    pub fn scale(&mut self, scale: &Vector3<f32>) {
        let scale = Matrix4::new_nonuniform_scaling(scale);
        self.scale = scale * self.scale;
    }

    pub fn rotate(&mut self, rotation: &Vector3<f32>) {
        let rotation = Matrix4::from_euler_angles(
            rotation.x.to_radians(),
            rotation.y.to_radians(),
            rotation.z.to_radians(),
        );
        self.rotation = rotation * self.rotation;
    }

    pub fn create_tbn(&mut self) {
        for polygon in self.polygons.iter_mut() {
            polygon.create_tbn();
        }
    }

    pub fn transform_to_world(&self) -> PolygonMesh {
        let mut polygons: Vec<Polygon> = Vec::new();
        let obj_to_world = self.obj_to_world_mtr();

        for polygon in self.polygons.iter() {
            let mut buff_polygon = polygon.clone();
            buff_polygon.move_to_center(&self.enclosing_sphere.center);
            buff_polygon.transform(&obj_to_world);
            buff_polygon.move_to_center(&-self.enclosing_sphere.center);
            polygons.push(buff_polygon);
        }

        PolygonMesh::new(
            polygons,
            self.color.clone(),
            self.kd.clone(),
            self.ks.clone(),
            self.kt.clone(),
            self.luminosity.clone(),
            self.texture.clone(),
            self.normal_map.clone(),
        )
    }
}
