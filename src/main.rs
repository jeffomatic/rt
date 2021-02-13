use core::f64;

mod camera;
mod hit;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hit::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let aspect: f64 = 16.0 / 9.0;
    let w = 400;
    let h = (w as f64 / aspect).round() as i32;
    let sampling_rate = 100;
    let camera = Camera::new(aspect);

    let s1 = Sphere {
        pos: Vec3::new(0.0, 0.0, -1.0),
        r: 0.5,
    };
    let s2 = Sphere {
        pos: Vec3::new(0.0, -100.5, -1.0),
        r: 100.0,
    };
    let world = HittableList::new(vec![&s1, &s2]);

    println!("P3");
    println!("{} {}", w, h);
    println!("255");

    for i in 0..h {
        for j in 0..w {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..sampling_rate {
                let u = (j as f64 + rand::random::<f64>()) / (w - 1) as f64;
                let v = ((h - i - 1) as f64 + rand::random::<f64>()) / (h - 1) as f64;
                let ray = camera.ray(u, v);
                color += ray_color(&ray, &world);
            }

            write_color((color / sampling_rate as f64).clamp(0.0, 0.999999));
        }
    }
}

fn ray_color(ray: &Ray, hittable: &dyn Hittable) -> Vec3 {
    if let Some(hit) = hittable.check_hit(&ray, 0.0, f64::MAX) {
        (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5 // project [-1, 1] into [0, 1]
    } else {
        // background
        let t = 0.5 * ray.dir.unit().y + 0.5;
        Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
    }
}

fn write_color(color: Vec3) {
    println!(
        "{} {} {}",
        (255.0 * color.x).round() as i32,
        (255.0 * color.y).round() as i32,
        (255.0 * color.z).round() as i32
    )
}
