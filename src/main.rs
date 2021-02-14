use core::f32;
use std::io::Write;

mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod util;

use camera::Camera;
use glam::Vec3A;
use hit::{Hittable, HittableList};
use material::Material;
use rand::random;
use ray::Ray;
use sphere::Sphere;
use util::{random_between, vec3a_random};

fn main() {
    let mut spheres: Vec<Sphere> = Vec::new();

    // Large planet/ground sphere
    spheres.push(Sphere {
        pos: Vec3A::new(0.0, -1000.0, 0.0),
        r: 1000.0,
        material: Material::Lambertian {
            albedo: Vec3A::new(0.5, 0.5, 0.5),
        },
    });

    // Random small spheres
    let span = 11;
    let center = Vec3A::new(4.0, 0.2, 0.0);

    for x in -span..span {
        for z in -span..span {
            let pos = Vec3A::new(
                x as f32 + 0.9 * random::<f32>(),
                0.2,
                z as f32 + 0.9 * random::<f32>(),
            );

            if (pos - center).length() <= 0.9 {
                continue;
            }

            let choose_mat = random::<f32>();

            if choose_mat < 0.8 {
                // lambertian
                spheres.push(Sphere {
                    pos,
                    r: 0.2,
                    material: Material::Lambertian {
                        albedo: vec3a_random(0.0, 1.0) * vec3a_random(0.0, 1.0),
                    },
                });
            } else if choose_mat < 0.95 {
                // metal
                spheres.push(Sphere {
                    pos,
                    r: 0.2,
                    material: Material::Metal {
                        albedo: vec3a_random(0.5, 1.0),
                        fuzz: random_between(0.0, 0.5),
                    },
                });
            } else {
                // dielectric
                spheres.push(Sphere {
                    pos,
                    r: 0.2,
                    material: Material::Dielectric { ir: 1.5 },
                });
            }
        }
    }

    // Large spheres
    spheres.push(Sphere {
        pos: Vec3A::new(0.0, 1.0, 0.0),
        r: 1.0,
        material: Material::Dielectric { ir: 1.5 },
    });
    spheres.push(Sphere {
        pos: Vec3A::new(-4.0, 1.0, 0.0),
        r: 1.0,
        material: Material::Lambertian {
            albedo: Vec3A::new(0.4, 0.2, 0.1),
        },
    });
    spheres.push(Sphere {
        pos: Vec3A::new(4.0, 1.0, 0.0),
        r: 1.0,
        material: Material::Metal {
            albedo: Vec3A::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    });

    let world = HittableList::new(
        spheres
            .iter()
            .map(|sphere| sphere as &dyn Hittable)
            .collect(),
    );

    let aspect: f32 = 3.0 / 2.0;
    let w = 1200;
    let h = (w as f32 / aspect).round() as i32;

    let pos = Vec3A::new(13.0, 2.0, 3.0);
    let target = Vec3A::new(0.0, 0.0, 0.0);
    let camera = Camera::new(camera::Config {
        pos,
        target,
        vup: Vec3A::new(0.0, 1.0, 0.0),
        vfov: 0.35,
        aspect,
        lens_radius: 0.05,
        focus_distance: 10.0,
    });

    println!("P3");
    println!("{} {}", w, h);
    println!("255");

    let sampling_rate = 500;
    let max_bounces = 50;

    for i in 0..h {
        let stderr = std::io::stderr();
        let mut handle = stderr.lock();
        handle
            .write_all(format!("\rscanlines remaining: {} ", h - i).as_bytes())
            .unwrap();

        for j in 0..w {
            let mut color = Vec3A::new(0.0, 0.0, 0.0);

            // average out multiple samples for antialiasing
            for _ in 0..sampling_rate {
                let u = (j as f32 + rand::random::<f32>()) / (w - 1) as f32;
                let v = ((h - i - 1) as f32 + rand::random::<f32>()) / (h - 1) as f32;
                let ray = camera.ray(u, v);
                color += ray_color(&ray, &world, max_bounces);
            }

            color /= sampling_rate as f32;

            // apply gamma correction
            color.x = color.x.sqrt().clamp(0.0, 0.999);
            color.y = color.y.sqrt().clamp(0.0, 0.999);
            color.z = color.z.sqrt().clamp(0.0, 0.999);

            write_color(color);
        }
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable, bounces: i32) -> Vec3A {
    if bounces <= 0 {
        return Vec3A::new(0.0, 0.0, 0.0);
    }

    // tmin is 0.001 to prevent "shadow acne"
    if let Some(hit) = world.check_hit(&ray, 0.001, f32::MAX) {
        if let Some(scatter) = hit.material.scatter(ray, &hit) {
            return scatter.attenuation * ray_color(&scatter.ray, world, bounces - 1);
        }

        return Vec3A::new(0.0, 0.0, 0.0);
    }

    // background
    let t = 0.5 * ray.dir.clone().normalize().y + 0.5;
    Vec3A::lerp(Vec3A::new(1.0, 1.0, 1.0), Vec3A::new(0.5, 0.7, 1.0), t)
}

fn write_color(color: Vec3A) {
    println!(
        "{} {} {}",
        (255.0 * color.x).round() as i32,
        (255.0 * color.y).round() as i32,
        (255.0 * color.z).round() as i32
    )
}
