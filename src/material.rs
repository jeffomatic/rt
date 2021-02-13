use crate::{
    hit::Hit,
    ray::Ray,
    util::{random_unit_sphere_offset, random_unit_vector},
    vec3::Vec3,
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(
        Vec3, // albedo
    ),
    Metal(
        Vec3, // albedo
        f64,  // fuzz
    ),
}

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult> {
        match self {
            Material::Lambertian(albedo) => {
                let mut dir = hit.normal + random_unit_vector();

                // Catch degenerate scatter direction
                if dir.near_zero() {
                    dir = hit.normal;
                }

                Some(ScatterResult {
                    attenuation: *albedo,
                    ray: Ray {
                        origin: hit.pos,
                        dir,
                    },
                })
            }
            Material::Metal(albedo, fuzz) => {
                let reflected = Vec3::reflect(ray_in.dir, hit.normal);
                let scattered = reflected + random_unit_sphere_offset() * *fuzz;

                // reflection needs to be in the same dir as normal
                if Vec3::dot(scattered, hit.normal) <= 0.0 {
                    return None;
                }

                Some(ScatterResult {
                    attenuation: *albedo,
                    ray: Ray {
                        origin: hit.pos,
                        dir: scattered,
                    },
                })
            }
        }
    }
}
