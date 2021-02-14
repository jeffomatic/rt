use glam::Vec3A;
use rand::random;

pub fn lerp(min: f32, max: f32, t: f32) -> f32 {
    (1.0 - t) * min + t * max
}

pub fn random_unit_sphere_offset() -> Vec3A {
    loop {
        let v = Vec3A::new(
            lerp(-1.0, 1.0, rand::random::<f32>()),
            lerp(-1.0, 1.0, rand::random::<f32>()),
            lerp(-1.0, 1.0, rand::random::<f32>()),
        );
        if v.length_squared() <= 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vector() -> Vec3A {
    random_unit_sphere_offset().normalize()
}

pub fn random_within_unit_disc() -> Vec3A {
    loop {
        let v = Vec3A::new(random_between(-1.0, 1.0), random_between(-1.0, 1.0), 0.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_between(min: f32, max: f32) -> f32 {
    min + (max - min) * random::<f32>()
}

pub fn vec3a_random(min: f32, max: f32) -> Vec3A {
    Vec3A::new(
        random_between(min, max),
        random_between(min, max),
        random_between(min, max),
    )
}

pub fn vec3a_near_zero(v: Vec3A) -> bool {
    let eps = 1e-8;
    v.x.abs() < eps && v.y.abs() < eps && v.z.abs() < eps
}

pub fn vec3a_reflect(v_in: Vec3A, normal: Vec3A) -> Vec3A {
    v_in - normal * 2.0 * v_in.dot(normal)
}
