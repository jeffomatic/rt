use glam::Vec3A;

use crate::{
    hit::{Hit, Hittable},
    material::Material,
    ray::Ray,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub pos: Vec3A,
    pub r: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn check_hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        let center_to_origin = ray.origin - self.pos;

        // Construct components of quadratic formula. This uses an algebraic
        // simplification that uses half_b, which is equal to b / 2.
        let a = ray.dir.dot(ray.dir);
        let half_b = Vec3A::dot(ray.dir, center_to_origin);
        let c = Vec3A::dot(center_to_origin, center_to_origin) - self.r * self.r;

        // d -> discriminant (portion within square root operator)
        let d = half_b * half_b - a * c;
        if d < 0.0 {
            // no roots
            return None;
        }

        let dsqrt = d.sqrt();

        // Find the first root within the acceptable range
        let mut root = (-half_b - dsqrt) / a;
        if root < tmin || tmax < root {
            root = (-half_b + dsqrt) / a;
            if root < tmin || tmax < root {
                return None;
            }
        }

        let hitpos = ray.at(root);
        let out_normal = (hitpos - self.pos) / self.r; // divide to get unit normal
        let normal_res = Hit::get_normal(&ray, &out_normal);

        Some(Hit {
            pos: hitpos,
            t: root,
            normal: normal_res.normal,
            front_face: normal_res.front_face,
            material: self.material,
        })
    }
}
