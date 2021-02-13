use crate::{
    hit::Hit,
    ray::Ray,
    util::{random_unit_sphere_offset, random_unit_vector},
    vec3::Vec3,
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { ir: f64 },
}

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult> {
        match self {
            Material::Lambertian { albedo } => {
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

            Material::Metal { albedo, fuzz } => {
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

            Material::Dielectric { ir } => {
                let refract_ratio = if hit.front_face { 1.0 / ir } else { *ir };
                let in_dir = ray_in.dir.unit();

                // theta is the angle of _incidence_.
                // TODO: is the min necessary here if both in_dir and hit.normal
                // are unit vectors? in_dir is a unit vector by initialization,
                // and the only thing that currently generates hit normals is
                // the sphere hittable, which returns unit vectors...
                let cos_theta = f64::min(Vec3::dot(-in_dir, hit.normal), 1.0);

                // Calculate the refraction vector as the sum two vectors:
                // - one vector orthogonal to the inverted surface normal
                // - one vector parallel to the inverted surface normal
                let refract_ortho = (in_dir + hit.normal * cos_theta) * refract_ratio;
                let refract_parallel =
                    hit.normal * -(1.0 as f64 - refract_ortho.length_squared()).abs().sqrt();

                Some(ScatterResult {
                    attenuation: Vec3::new(1.0, 1.0, 1.0),
                    ray: Ray {
                        origin: hit.pos,
                        dir: refract_ortho + refract_parallel,
                    },
                })
            }
        }
    }
}
