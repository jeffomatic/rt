use core::f64;
use std::io::Write;

mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;

use camera::Camera;
use hit::{Hittable, HittableList};
use material::Material;
use rand::random;
use ray::Ray;
use sphere::Sphere;
use util::random_between;
use vec3::Vec3;

fn main() {
    let mut spheres: Vec<Sphere> = Vec::new();

    // Large planet/ground sphere
    spheres.push(Sphere {
        pos: Vec3::new(0.0, -1000.0, 0.0),
        r: 1000.0,
        material: Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    });

    // Random small spheres
    let span = 11;
    let center = Vec3::new(4.0, 0.2, 0.0);

    for x in -span..span {
        for z in -span..span {
            let pos = Vec3::new(
                x as f64 + 0.9 * random::<f64>(),
                0.2,
                z as f64 + 0.9 * random::<f64>(),
            );

            if (pos - center).length() <= 0.9 {
                continue;
            }

            let choose_mat = random::<f64>();

            if choose_mat < 0.8 {
                // lambertian
                spheres.push(Sphere {
                    pos,
                    r: 0.2,
                    material: Material::Lambertian {
                        albedo: Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0),
                    },
                });
            } else if choose_mat < 0.95 {
                // metal
                spheres.push(Sphere {
                    pos,
                    r: 0.2,
                    material: Material::Metal {
                        albedo: Vec3::random(0.5, 1.0),
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
        pos: Vec3::new(0.0, 1.0, 0.0),
        r: 1.0,
        material: Material::Dielectric { ir: 1.5 },
    });
    spheres.push(Sphere {
        pos: Vec3::new(-4.0, 1.0, 0.0),
        r: 1.0,
        material: Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    });
    spheres.push(Sphere {
        pos: Vec3::new(4.0, 1.0, 0.0),
        r: 1.0,
        material: Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    });

    let world = HittableList::new(
        spheres
            .iter()
            .map(|sphere| sphere as &dyn Hittable)
            .collect(),
    );

    let aspect: f64 = 3.0 / 2.0;
    let w = 1200;
    let h = (w as f64 / aspect).round() as i32;

    let pos = Vec3::new(13.0, 2.0, 3.0);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(camera::Config {
        pos,
        target,
        vup: Vec3::new(0.0, 1.0, 0.0),
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
            let mut color = Vec3::new(0.0, 0.0, 0.0);

            // average out multiple samples for antialiasing
            for _ in 0..sampling_rate {
                let u = (j as f64 + rand::random::<f64>()) / (w - 1) as f64;
                let v = ((h - i - 1) as f64 + rand::random::<f64>()) / (h - 1) as f64;
                let ray = camera.ray(u, v);
                color += ray_color(&ray, &world, max_bounces);
            }

            color /= sampling_rate as f64;

            // apply gamma correction
            color = color.sqrt();

            write_color(color.clamp(0.0, 0.999));
        }
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable, bounces: i32) -> Vec3 {
    if bounces <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // tmin is 0.001 to prevent "shadow acne"
    if let Some(hit) = world.check_hit(&ray, 0.001, f64::MAX) {
        if let Some(scatter) = hit.material.scatter(ray, &hit) {
            return scatter.attenuation * ray_color(&scatter.ray, world, bounces - 1);
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    // background
    let t = 0.5 * ray.dir.unit().y + 0.5;
    Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

fn write_color(color: Vec3) {
    println!(
        "{} {} {}",
        (255.0 * color.x).round() as i32,
        (255.0 * color.y).round() as i32,
        (255.0 * color.z).round() as i32
    )
}
