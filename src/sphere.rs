use crate::{
    hit::{Hit, Hittable},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub pos: Vec3,
    pub r: f64,
}

impl Hittable for Sphere {
    fn check_hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Hit> {
        let center_to_origin = ray.origin - self.pos;

        // construct components of quadratic formula
        let a = Vec3::dot(ray.dir, ray.dir);
        let b = 2.0 * Vec3::dot(ray.dir, center_to_origin);
        let c = Vec3::dot(center_to_origin, center_to_origin) - self.r * self.r;

        // d -> discriminant (portion within square root operator)
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            // no roots
            return None;
        }

        let dsqrt = d.sqrt();

        // Find the first root within the acceptable range
        let mut root = (-b - dsqrt) / (2.0 * a);
        if root < tmin || tmax < root {
            root = (-b + dsqrt) / (2.0 * a);
            if root < tmin || tmax < root {
                return None;
            }
        }

        let hitpos = ray.at(root);
        let out_normal = (hitpos - self.pos) / self.r;
        let normal_res = Hit::get_normal(&ray, &out_normal);

        Some(Hit {
            pos: hitpos,
            t: root,
            normal: normal_res.normal,
            front_face: normal_res.front_face,
        })
    }
}
