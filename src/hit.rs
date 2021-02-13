use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn check_hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Hit>;
}
