use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn check_hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Hit>;
}

pub struct NormalInfo {
    pub normal: Vec3,
    pub front_face: bool,
}

impl Hit {
    pub fn get_normal(ray: &Ray, out_normal: &Vec3) -> NormalInfo {
        let front_face = Vec3::dot(ray.dir, *out_normal) < 0.0;
        NormalInfo {
            normal: if front_face {
                out_normal.clone()
            } else {
                -out_normal.clone()
            },
            front_face,
        }
    }
}

pub struct HittableList<'a>(Vec<&'a dyn Hittable>);

impl Hittable for HittableList<'_> {
    fn check_hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Hit> {
        let mut res: Option<Hit> = None;
        let mut best_dist = tmax;

        for h in self.0.iter() {
            if let Some(hit) = h.check_hit(ray, tmin, best_dist) {
                res = Some(hit);
                best_dist = hit.t;
            }
        }

        res
    }
}

impl HittableList<'_> {
    pub fn new<'a>(hittables: Vec<&'a dyn Hittable>) -> HittableList<'a> {
        HittableList(hittables)
    }
}
