use crate::camera::FovCamera;
use crate::light::DistantLight;
use crate::polygon::RayIntersect;
use crate::polygon::polygon_mesh::polygon_meshes::PolygonMeshes;
use egui::{Color32, ColorImage};
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;

const BIAS: f32 = 1e-4;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
    pub depth: i32,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin,
            direction,
            depth: 1,
        }
    }
    pub fn render(
        objects: &PolygonMeshes,
        fov_camera: &FovCamera,
        lights: &Vec<DistantLight>,
        image: &mut ColorImage,
    ) {
        
        let objects = objects.transform_to_world();
        
        let rad_fov = (fov_camera.fov * 0.5).to_radians();
        let scale = f32::tan(rad_fov);

        let origin = fov_camera.origin;
        let mut pixels = vec![Color32::default(); image.width() * image.height()];

        pixels
            .par_chunks_mut(image.width())
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..image.width() {
                    let dx = x as f32 - image.width() as f32 / 2.0;
                    let dy = image.height() as f32 / 2.0 - y as f32;
                    let dz = -(image.height() as f32 / 2.0) / scale;
                    let dir = Vector3::new(dx, dy, dz);

                    let dir_cam = (Vector3::from(
                        (fov_camera.camera_to_world(Some(&Vector3::new(0.0, 0.0, 1.0)))
                            * dir.to_homogeneous())
                        .xyz(),
                    ) - origin.coords)
                        .normalize();

                    let ray = Ray::new(origin, dir_cam);
                    let (color, _) = ray.cast(&objects, lights, 3, None);

                    row[x] = Color32::from_rgb(color[0], color[1], color[2]);
                }
            });

        for y in 0..image.height() {
            for x in 0..image.width() {
                image[(x, y)] = pixels[y * image.width() + x];
            }
        }
    }

    pub fn ray_intersection(&self, objects: &PolygonMeshes) -> Option<(RayIntersect, usize)> {
        let mut t_min = f32::INFINITY;
        let mut intersection_data: Option<RayIntersect> = None;
        let mut object_id: usize = 0;

        for (idx, object) in objects.meshes.iter().enumerate() {
            if !object.sphere_intersect(self) {
                continue;
            }

            let hit = object.intersect(self);
            let buff_hit = hit.clone();

            if let Some(hit) = hit {
                if hit.t < t_min {
                    t_min = hit.t;
                    intersection_data = buff_hit;
                    object_id = idx;
                }
            }
        }

        if let Some(intersection) = intersection_data {
            Some((intersection, object_id))
        } else {
            None
        }
    }

    fn reflected_ray(l: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        let beta = 2.0 * l.dot(&n);

        (beta * n - l).normalize()
    }

    fn refracted_ray(l: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        let nu: f32 = 1.0 / 1.0;
        let cos_i = -l.dot(&n).min(1.0).max(-1.0);
        if cos_i < 0.0 {
            return Self::refracted_ray(l, &-n);
        }

        let k = 1.0 - nu * nu * (1.0 - cos_i * cos_i);

        if k < 0.0 {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            l * nu + n * (nu * cos_i - k.sqrt())
        }
    }

    fn intersection_distance(point_1: &RayIntersect, point_2: &RayIntersect) -> f32 {
        nalgebra::distance(&point_1.intersection_point, &point_2.intersection_point) + 1.0
    }

    pub fn cast(
        &self,
        objects: &PolygonMeshes,
        lights: &Vec<DistantLight>,
        max_depth: i32,
        prev_intersection: Option<&RayIntersect>,
    ) -> (Vec<u8>, bool) {
        let mut color = vec![0, 0, 0];
        let background_color = vec![0, 0, 0];
        let mut is_hit = false;

        if let Some((hit, idx)) = self.ray_intersection(objects) {
            is_hit = true;
            let n = hit.surface_normal;
            let s = -self.direction;
            let object = objects.get_object(idx);
            let mut object_color = object.color.clone();
            if let Some(color) = hit.texture_color {
                object_color = vec![color[0], color[1], color[2]];
            }

            let reflected_dir = Self::reflected_ray(&s, &n);
            let refracted_dir = Self::refracted_ray(&self.direction, &n);

            let mut refracted_origin = hit.intersection_point + BIAS * n;
            if refracted_dir.dot(&n) < 0.0 {
                refracted_origin = hit.intersection_point - BIAS * n;
            }

            let mut reflected_origin = hit.intersection_point + BIAS * n;
            if reflected_dir.dot(&n) < 0.0 {
                reflected_origin = hit.intersection_point - BIAS * n;
            }

            let mut reflected_ray = Ray::new(reflected_origin, reflected_dir);
            let mut refracted_ray = Ray::new(refracted_origin, refracted_dir);

            reflected_ray.depth = self.depth + 1;
            refracted_ray.depth = self.depth + 1;

            if self.depth < max_depth {
                let (reflected_color, reflected_hit) =
                    Ray::cast(&reflected_ray, objects, lights, max_depth, Some(&hit));
                if reflected_hit {
                    color[0] += (object.ks[0] * reflected_color[0] as f32) as u8;
                    color[1] += (object.ks[1] * reflected_color[1] as f32) as u8;
                    color[2] += (object.ks[2] * reflected_color[2] as f32) as u8;
                }

                let (refracted_color, refracted_hit) =
                    Ray::cast(&refracted_ray, objects, lights, max_depth, Some(&hit));
                if refracted_hit {
                    color[0] += (object.kt[0] * refracted_color[0] as f32) as u8;
                    color[1] += (object.kt[1] * refracted_color[1] as f32) as u8;
                    color[2] += (object.kt[2] * refracted_color[2] as f32) as u8;
                }
            }

            let mut color_r = 0.0;
            let mut color_g = 0.0;
            let mut color_b = 0.0;

            for light in lights {
                let l = light.vector_to_light(&hit.intersection_point);
                let mut shadow_origin = hit.intersection_point + BIAS * n;
                if l.dot(&n) < 0.0 {
                    shadow_origin = hit.intersection_point - BIAS * n;
                }
                let shadow_ray = Ray::new(shadow_origin, l);
                let mut shadow_param = 1.0;

                if let Some((hit, _)) = shadow_ray.ray_intersection(objects) {
                    if (hit.intersection_point - shadow_origin).norm()
                        < (light.origin - shadow_origin).norm()
                    {
                        shadow_param = 0.0;
                    }
                }
                let r = Self::reflected_ray(&l, &n);
                
                let diffuse = n.dot(&l).max(0.0) * light.intensity * shadow_param;
                let specular =
                    r.dot(&n).max(0.0).powi(object.luminosity) * light.intensity * shadow_param;
                let background = light.ka * light.back_intensity;

                color_r += background * light.color[0] as f32
                    + (object_color[0] as f32 * object.kd[0] * diffuse
                        + object.ks[0] * specular * light.color[0] as f32);
                color_g += background * light.color[1] as f32
                    + (object_color[1] as f32 * object.kd[1] * diffuse
                        + object.ks[1] * specular * light.color[1] as f32);
                color_b += background * light.color[2] as f32
                    + (object_color[2] as f32 * object.kd[2] * diffuse
                        + object.ks[2] * specular * light.color[2] as f32);
            }

            if let Some(intersection) = prev_intersection {
                let d = Ray::intersection_distance(&hit, &intersection);
                color_r /= d;
                color_g /= d;
                color_b /= d;
            }

            color[0] = (color[0] as usize + color_r.clamp(0.0, 255.0) as usize).clamp(0, 255) as u8;
            color[1] = (color[1] as usize + color_g.clamp(0.0, 255.0) as usize).clamp(0, 255) as u8;
            color[2] = (color[2] as usize + color_b.clamp(0.0, 255.0) as usize).clamp(0, 255) as u8;
            
            return (color, is_hit);
        }

        (background_color, is_hit)
    }
}
