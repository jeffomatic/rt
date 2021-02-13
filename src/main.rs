use core::f64;

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
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let aspect: f64 = 16.0 / 9.0;
    let w = 400;
    let h = (w as f64 / aspect).round() as i32;
    let sampling_rate = 100;
    let max_bounces = 50;

    let mat_ground = Material::Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let mat_center = Material::Dielectric { ir: 1.5 };
    let mat_left = Material::Dielectric { ir: 1.5 };
    let mat_right = Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    let ground = Sphere {
        pos: Vec3::new(0.0, -100.5, -1.0),
        r: 100.0,
        material: mat_ground,
    };
    let center = Sphere {
        pos: Vec3::new(0.0, 0.0, -1.0),
        r: 0.5,
        material: mat_center,
    };
    let left = Sphere {
        pos: Vec3::new(-1.0, 0.0, -1.0),
        r: 0.5,
        material: mat_left,
    };
    let right = Sphere {
        pos: Vec3::new(1.0, 0.0, -1.0),
        r: 0.5,
        material: mat_right,
    };

    let world = HittableList::new(vec![&ground, &center, &left, &right]);
    let camera = Camera::new(aspect);

    println!("P3");
    println!("{} {}", w, h);
    println!("255");

    for i in 0..h {
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
