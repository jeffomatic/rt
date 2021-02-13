use core::f64;

mod hit;
mod ray;
mod sphere;
mod vec3;

use hit::{Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let aspect: f64 = 16.0 / 9.0;
    let w = 400;
    let h = (w as f64 / aspect).round() as i32;

    let view_height = 2.0;
    let view_width = view_height * aspect;
    let flength = 1.0;

    let world_origin = Vec3::new(0.0, 0.0, 0.0);
    let view_x = Vec3::new(view_width as f64, 0.0, 0.0);
    let view_y = Vec3::new(0.0, view_height, 0.0);
    let view_origin = world_origin - view_x / 2.0 - view_y / 2.0 + Vec3::new(0.0, 0.0, -flength);

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
            let u = j as f64 / (w - 1) as f64;
            let v = (h - i - 1) as f64 / (h - 1) as f64;
            let view_pos = view_origin + view_x * u + view_y * v;
            let ray = Ray {
                origin: world_origin,
                dir: view_pos - world_origin,
            };

            let color = if let Some(hit) = world.check_hit(&ray, 0.0, f64::MAX) {
                (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5 // project [-1, 1] into [0, 1]
            } else {
                // background
                let t = 0.5 * ray.dir.unit().y + 0.5;
                Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
            };

            write_color(color);
        }
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
