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
    let max_bounces = 50;

    let s1 = Sphere {
        pos: Vec3::new(0.0, 0.0, -1.0),
        r: 0.5,
    };
    let s2 = Sphere {
        pos: Vec3::new(0.0, -100.5, -1.0),
        r: 100.0,
    };
    let world = HittableList::new(vec![&s1, &s2]);
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
            color.x = color.x.sqrt();
            color.y = color.y.sqrt();
            color.z = color.z.sqrt();

            write_color(color.clamp(0.0, 0.999));
        }
    }
}

fn ray_color(ray: &Ray, hittable: &dyn Hittable, bounces: i32) -> Vec3 {
    if bounces <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // tmin is 0.001 to prevent "shadow acne"
    if let Some(hit) = hittable.check_hit(&ray, 0.001, f64::MAX) {
        // diffuse lighting:
        // recursively determine color light ray approaching the hit location,
        // using a random direction. We choose this direction by picking a point
        // on the surface of the unit sphere tangent to the hit point, and
        // finding the ray from the original reflection point to the randomly-
        // chosen point.
        let target = hit.pos + hit.normal + random_unit_sphere_offset().unit();
        let next_ray = Ray {
            origin: hit.pos,
            dir: target - hit.pos,
        };

        // objects are considered 50% reflectors
        return ray_color(&next_ray, hittable, bounces - 1) * 0.5;
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

fn random_unit_sphere_offset() -> Vec3 {
    loop {
        let v = Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        );
        if v.length_squared() <= 1.0 {
            return v;
        }
    }
}
