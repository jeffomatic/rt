use glam::Vec3A;
use rand::random;

use crate::{
    hit::Hit,
    ray::Ray,
    util::{random_unit_sphere_offset, random_unit_vector, vec3a_near_zero, vec3a_reflect},
};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Vec3A },
    Metal { albedo: Vec3A, fuzz: f32 },
    Dielectric { ir: f32 },
}

pub struct ScatterResult {
    pub attenuation: Vec3A,
    pub ray: Ray,
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<ScatterResult> {
        match self {
            Material::Lambertian { albedo } => {
                let mut dir = hit.normal + random_unit_vector();

                // Catch degenerate scatter direction
                if vec3a_near_zero(dir) {
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
                let reflected = vec3a_reflect(ray_in.dir, hit.normal);
                let scattered = reflected + random_unit_sphere_offset() * *fuzz;

                // reflection needs to be in the same dir as normal
                if Vec3A::dot(scattered, hit.normal) <= 0.0 {
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
                let in_dir = ray_in.dir.clone().normalize();

                // theta is the angle of _incidence_.
                // TODO: is the min necessary here if both in_dir and hit.normal
                // are unit vectors? in_dir is a unit vector by initialization,
                // and the only thing that currently generates hit normals is
                // the sphere hittable, which returns unit vectors...
                let cos_theta = f32::min(Vec3A::dot(-in_dir, hit.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                // We determine whether we refract or reflect based on whether
                // there is a solution to Snell's Law. If there isn't, then we
                // have to reflect.
                let no_snell_solution = refract_ratio * sin_theta > 1.0;

                // Some materials make it more likely to reflect at steep
                // angles. We'll use Schlick's approximation to determine the
                // probability, and randomly insert reflections.
                let r0 = (1.0 - ir) / (1.0 + ir);
                let r0_2 = r0 * r0;
                let reflectance = r0_2 + (1.0 - r0_2) * (1.0 - cos_theta).powf(5.0);

                let scatter_dir = if no_snell_solution || random::<f32>() < reflectance {
                    // reflection
                    vec3a_reflect(in_dir, hit.normal)
                } else {
                    // refraction
                    // Calculate the refraction vector as the sum two vectors:
                    // - one vector orthogonal to the inverted surface normal
                    // - one vector parallel to the inverted surface normal
                    let refract_ortho = (in_dir + hit.normal * cos_theta) * refract_ratio;
                    let refract_parallel =
                        hit.normal * -(1.0 as f32 - refract_ortho.length_squared()).abs().sqrt();

                    refract_ortho + refract_parallel
                };

                Some(ScatterResult {
                    attenuation: Vec3A::new(1.0, 1.0, 1.0),
                    ray: Ray {
                        origin: hit.pos,
                        dir: scatter_dir,
                    },
                })
            }
        }
    }
}
