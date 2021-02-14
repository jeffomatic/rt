use rand::random;

use crate::vec3::Vec3;

pub fn lerp(min: f64, max: f64, t: f64) -> f64 {
    (1.0 - t) * min + t * max
}

pub fn random_unit_sphere_offset() -> Vec3 {
    loop {
        let v = Vec3::new(
            lerp(-1.0, 1.0, rand::random::<f64>()),
            lerp(-1.0, 1.0, rand::random::<f64>()),
            lerp(-1.0, 1.0, rand::random::<f64>()),
        );
        if v.length_squared() <= 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_unit_sphere_offset().unit()
}

pub fn random_within_unit_disk() -> Vec3 {
    loop {
        let v = Vec3::new(
            random::<f64>() * 2.0 - 1.0,
            random::<f64>() * 2.0 - 1.0,
            0.0,
        );
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}
