use crate::camera::FovCamera;
use crate::light::DistantLight;
use crate::light::lights::Lights;
use crate::polygon::RayIntersect;
use crate::polygon::polygon_mesh::PolygonMesh;
use crate::polygon::polygon_mesh::polygon_meshes::PolygonMeshes;
use crate::texture::Texture;
use egui::{Color32, ColorImage};
use nalgebra::{Point3, Vector3};
use rayon::prelude::*;

const BIAS: f32 = 1e-4;

#[derive(Clone, Debug)]
pub struct Differentials(pub Vector3<f32>, pub Vector3<f32>);

#[derive(Clone, Debug)]
pub struct IntersectionDifferentials(pub f32, pub f32);

impl Default for Differentials {
    fn default() -> Self {
        Differentials(Vector3::zeros(), Vector3::zeros())
    }
}

impl Default for IntersectionDifferentials {
    fn default() -> Self {
        IntersectionDifferentials(0.0, 0.0)
    }
}

#[derive(Clone, Debug)]
pub struct RayDifferentials {
    pub d_d: Differentials,
    pub d_o: Option<Differentials>,
    pub non_norm_direction: Vector3<f32>,
    pub du: IntersectionDifferentials,
    pub dv: IntersectionDifferentials,
    pub e1: Vector3<f32>,
    pub e2: Vector3<f32>,
    pub dn: Differentials,
    pub d_s: IntersectionDifferentials,
    pub d_t: IntersectionDifferentials,
}

impl Default for RayDifferentials {
    fn default() -> Self {
        RayDifferentials {
            d_d: Differentials::default(),
            d_o: None,
            non_norm_direction: Vector3::zeros(),
            du: IntersectionDifferentials::default(),
            dv: IntersectionDifferentials::default(),
            e1: Vector3::zeros(),
            e2: Vector3::zeros(),
            dn: Differentials::default(),
            d_s: IntersectionDifferentials::default(),
            d_t: IntersectionDifferentials::default(),
        }
    }
}

impl RayDifferentials {
    pub fn new(
        d_d: Differentials,
        d_o: Option<Differentials>,
        non_norm_direction: Vector3<f32>,
        du: IntersectionDifferentials,
        dv: IntersectionDifferentials,
        e1: Vector3<f32>,
        e2: Vector3<f32>,
    ) -> Self {
        Self {
            d_d,
            d_o,
            non_norm_direction,
            du,
            dv,
            e1,
            e2,
            dn: Differentials::default(),
            d_s: IntersectionDifferentials::default(),
            d_t: IntersectionDifferentials::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
    pub depth: i32,
    pub differentials: RayDifferentials,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin,
            direction,
            depth: 1,
            differentials: RayDifferentials::default(),
        }
    }
    pub fn render(
        objects: &PolygonMeshes,
        fov_camera: &FovCamera,
        lights: &Lights,
        image: &mut ColorImage,
        bg_color: Vec<u8>,
        trace_depth: i32,
    ) {
        let objects = objects.transform_to_world();

        let rad_fov = (fov_camera.fov * 0.5).to_radians();
        let scale = f32::tan(rad_fov);

        let origin = fov_camera.origin;
        let mut pixels = vec![Color32::default(); image.width() * image.height()];

        let aspect_ratio = image.width() as f32 / image.height() as f32;

        let view = (fov_camera.target - fov_camera.origin).normalize();
        let right = Vector3::new(0.0, 0.0, 1.0).cross(&view).normalize();
        let up = view.cross(&right).normalize();

        let view_dir = -view;
        let right_dir = aspect_ratio * scale * right;
        let up_dir = -scale * up;

        pixels
            .par_chunks_mut(image.width())
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..image.width() {
                    let dir_world = view_dir
                        + ((2.0 * x as f32 + 1.0) / image.width() as f32 - 1.0) * right_dir
                        + ((2.0 * y as f32 + 1.0) / image.height() as f32 - 1.0) * up_dir;

                    let r = ((2.0 * aspect_ratio * scale) / image.width() as f32) * right;
                    let u = -((2.0 * scale) / image.height() as f32) * up;
                    let dot_product = dir_world.dot(&dir_world);
                    let dot_product_power = (dot_product * dot_product * dot_product).sqrt();

                    let dd_dx =
                        ((dot_product * r) - ((dir_world.dot(&r)) * dir_world)) / dot_product_power;
                    let dd_dy =
                        ((dot_product * u) - (dir_world.dot(&u) * dir_world)) / dot_product_power;

                    let mut ray = Ray::new(origin, dir_world.normalize());
                    ray.differentials.d_d = Differentials(dd_dx, dd_dy);
                    ray.differentials.non_norm_direction = dir_world;
                    ray.differentials.d_o = Some(Differentials(Vector3::zeros(), Vector3::zeros()));

                    let (color, _) = ray.cast(&objects, lights, trace_depth, None, bg_color.clone());
                    let srgb_color = Texture::linear_to_srgb(&color);

                    row[x] = Color32::from_rgb(srgb_color[0], srgb_color[1], srgb_color[2]);
                }
            });

        for y in 0..image.height() {
            for x in 0..image.width() {
                image[(x, y)] = pixels[y * image.width() + x];
            }
        }
    }

    pub fn ray_intersection(&mut self, objects: &PolygonMeshes) -> Option<(RayIntersect, usize)> {
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

    pub fn shadow_ray_intersection(
        &mut self,
        objects: &PolygonMeshes,
        light: &DistantLight,
    ) -> Vec<f32> {
        let mut shadow_param = vec![1.0, 1.0, 1.0];

        for object in objects.meshes.iter() {
            if !object.sphere_intersect(self) {
                continue;
            }

            let hit = object.intersect(self);
            if let Some(hit) = hit {
                if (hit.intersection_point - self.origin).norm()
                    < (light.origin - self.origin).norm()
                {
                    shadow_param[0] *= object.kt[0];
                    shadow_param[1] *= object.kt[1];
                    shadow_param[2] *= object.kt[2];
                }
            }
        }

        shadow_param
    }

    fn reflected_ray(l: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        let beta = 2.0 * l.dot(&n);

        beta * n - l
    }

    fn refracted_ray(l: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        let nu: f32 = 1.0;
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

    fn reflected_differentials(&mut self, n: &Vector3<f32>) -> Differentials {
        let dd_dn_dx =
            self.differentials.d_d.0.dot(&n) + self.direction.dot(&self.differentials.dn.0);
        let dd_dn_dy =
            self.differentials.d_d.1.dot(&n) + self.direction.dot(&self.differentials.dn.1);
        let n_dot_dir = n.dot(&self.direction);

        let dd_dx =
            self.differentials.d_d.0 - 2.0 * (n_dot_dir * self.differentials.dn.0 + dd_dn_dx * n);
        let dd_dy =
            self.differentials.d_d.1 - 2.0 * (n_dot_dir * self.differentials.dn.1 + dd_dn_dy * n);

        Differentials(dd_dx, dd_dy)
    }

    fn refracted_differentials(
        &mut self,
        n: &Vector3<f32>,
        refracted_dir: &Vector3<f32>,
    ) -> Differentials {
        let dir_dot_n = self.direction.dot(&n);
        let ref_dot_n = refracted_dir.dot(&n);

        let dd_dn_dx =
            self.differentials.d_d.0.dot(&n) + self.direction.dot(&self.differentials.dn.0);
        let dd_dn_dy =
            self.differentials.d_d.1.dot(&n) + self.direction.dot(&self.differentials.dn.1);

        let mu = dir_dot_n - ref_dot_n;

        let dmu_dx = (1.0 - dir_dot_n / ref_dot_n) * dd_dn_dx;
        let dmu_dy = (1.0 - dir_dot_n / ref_dot_n) * dd_dn_dy;

        let dd_dx = self.differentials.d_d.0 - (mu * self.differentials.dn.0 + dmu_dx * n);
        let dd_dy = self.differentials.d_d.1 - (mu * self.differentials.dn.1 + dmu_dy * n);

        Differentials(dd_dx, dd_dy)
    }

    pub fn secondary_rays(
        &mut self,
        objects: &PolygonMeshes,
        lights: &Lights,
        max_depth: i32,
        hit: &RayIntersect,
        bg_color: Vec<u8>,
        object: &PolygonMesh,
    ) -> Vec<f32> {
        let mut color = vec![0.0, 0.0, 0.0];
        let s = -self.direction;
        let n = hit.surface_normal;
        let background_color = Texture::srgb_to_linear(&bg_color);

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

        let mut reflected_ray = Ray::new(reflected_origin, reflected_dir.normalize());
        reflected_ray.differentials.d_o = None;

        let mut refracted_ray = Ray::new(refracted_origin, refracted_dir.normalize());
        refracted_ray.differentials.d_o = None;

        reflected_ray.depth = self.depth + 1;
        refracted_ray.depth = self.depth + 1;

        let reflected_d_dir = self.reflected_differentials(&n);
        let refracted_d_dir = self.refracted_differentials(&n, &refracted_dir);

        let reflected_differentials = RayDifferentials::new(
            reflected_d_dir,
            None,
            reflected_dir,
            self.differentials.du.clone(),
            self.differentials.dv.clone(),
            self.differentials.e1.clone(),
            self.differentials.e2.clone(),
        );
        let refracted_differentials = RayDifferentials::new(
            refracted_d_dir,
            None,
            refracted_dir,
            self.differentials.du.clone(),
            self.differentials.dv.clone(),
            self.differentials.e1.clone(),
            self.differentials.e2.clone(),
        );

        reflected_ray.differentials = reflected_differentials;
        refracted_ray.differentials = refracted_differentials;

        if self.depth < max_depth {
            if object.ks[0] != 0.0 || object.ks[1] != 0.0 || object.ks[2] != 0.0 {
                let (reflected_color, reflected_hit) = Ray::cast(
                    &mut reflected_ray,
                    objects,
                    lights,
                    max_depth,
                    Some(&hit),
                    bg_color.clone(),
                );
                if reflected_hit {
                    color[0] += object.ks[0] * reflected_color[0];
                    color[1] += object.ks[1] * reflected_color[1];
                    color[2] += object.ks[2] * reflected_color[2];
                } else {
                    color[0] += object.ks[0] * background_color[0];
                    color[1] += object.ks[1] * background_color[1];
                    color[2] += object.ks[2] * background_color[2];
                }
            }

            if object.kt[0] != 0.0 || object.kt[1] != 0.0 || object.kt[2] != 0.0 {
                let (refracted_color, refracted_hit) = Ray::cast(
                    &mut refracted_ray,
                    objects,
                    lights,
                    max_depth,
                    Some(&hit),
                    bg_color.clone(),
                );
                if refracted_hit {
                    color[0] += object.kt[0] * refracted_color[0];
                    color[1] += object.kt[1] * refracted_color[1];
                    color[2] += object.kt[2] * refracted_color[2];
                } else {
                    color[0] += object.kt[0] * background_color[0];
                    color[1] += object.kt[1] * background_color[1];
                    color[2] += object.kt[2] * background_color[2];
                }
            }
        }

        color
    }

    pub fn cast(
        &mut self,
        objects: &PolygonMeshes,
        lights: &Lights,
        max_depth: i32,
        prev_intersection: Option<&RayIntersect>,
        bg_color: Vec<u8>,
    ) -> (Vec<f32>, bool) {
        let mut is_hit = false;

        if let Some((hit, idx)) = self.ray_intersection(objects) {
            is_hit = true;
            let n = hit.surface_normal;
            let s = -self.direction;
            let object = objects.get_object(idx);

            let object_color_srgb = object.color.clone();
            let mut object_color = Texture::srgb_to_linear(&object_color_srgb);

            if let Some(color) = hit.texture_color {
                object_color = Texture::srgb_to_linear(&vec![color[0], color[1], color[2]]);
            }

            let mut color =
                self.secondary_rays(objects, lights, max_depth, &hit, bg_color, &object);

            let mut color_r = 0.0;
            let mut color_g = 0.0;
            let mut color_b = 0.0;

            for light in lights.lights.iter() {
                let l = light.vector_to_light(&hit.intersection_point);
                let mut shadow_origin = hit.intersection_point + BIAS * n;
                if l.dot(&n) < 0.0 {
                    shadow_origin = hit.intersection_point - BIAS * n;
                }
                let mut shadow_ray = Ray::new(shadow_origin, l);
                let shadow_param = shadow_ray.shadow_ray_intersection(objects, light);
                let r = Self::reflected_ray(&l, &n).normalize();

                let diffuse = n.dot(&l).max(0.0) * light.intensity;
                let specular = r.dot(&s).max(0.0).powi(object.luminosity) * light.intensity;
                let light_color = Texture::srgb_to_linear(&light.color);

                color_r += object_color[0] * object.kd[0] * diffuse * shadow_param[0]
                    + object.ks[0] * specular * light_color[0] * shadow_param[0];
                color_g += object_color[1] * object.kd[1] * diffuse * shadow_param[1]
                    + object.ks[1] * specular * light_color[1] * shadow_param[1];
                color_b += object_color[2] * object.kd[2] * diffuse * shadow_param[2]
                    + object.ks[2] * specular * light_color[2] * shadow_param[2];
            }

            let background = lights.ka * lights.back_intensity;
            let back_light_color = Texture::srgb_to_linear(&lights.back_color);

            color_r += background * back_light_color[0];
            color_g += background * back_light_color[1];
            color_b += background * back_light_color[2];

            if let Some(intersection) = prev_intersection {
                let d = Ray::intersection_distance(&hit, &intersection);
                color_r /= d;
                color_g /= d;
                color_b /= d;
            }

            color[0] += color_r;
            color[1] += color_g;
            color[2] += color_b;

            return (color, is_hit);
        }

        (Texture::srgb_to_linear(&bg_color), is_hit)
    }
}
