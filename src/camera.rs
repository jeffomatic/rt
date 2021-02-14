use crate::{ray::Ray, util::random_within_unit_disk, vec3::Vec3};

pub struct Camera {
    pos: Vec3,
    view_origin: Vec3,
    view_up: Vec3,
    view_right: Vec3,
    view_x: Vec3, // view_right, scaled by focus distance and viewport width
    view_y: Vec3, // view_up, scaled by focus distance and viewport height
    lens_radius: f64,
}

pub struct Config {
    pub pos: Vec3,
    pub target: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aspect: f64,
    pub lens_radius: f64, // half the aperture, which is a diameter
    pub focus_distance: f64,
}

impl Camera {
    pub fn new(config: Config) -> Camera {
        let view_height = 2.0 * (config.vfov / 2.0).tan();
        let view_width = view_height * config.aspect;

        let lookdir = (config.target - config.pos).unit();
        let view_right = Vec3::cross(lookdir, config.vup).unit();
        let view_up = Vec3::cross(view_right, lookdir);

        let pos = config.pos;
        let view_x = view_right * view_width * config.focus_distance;
        let view_y = view_up * view_height * config.focus_distance;
        let view_origin = pos - view_x / 2.0 - view_y / 2.0 + lookdir * config.focus_distance;

        Camera {
            pos,
            view_origin,
            view_x,
            view_y,
            view_up,
            view_right,
            lens_radius: config.lens_radius,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        // A random offset of the ray origin provides for defocus blurring.
        let offset_factor = random_within_unit_disk() * self.lens_radius;
        let offset = self.view_right * offset_factor.x + self.view_up * offset_factor.y;

        let view_origin = self.pos + offset;
        let view_pos = self.view_origin + self.view_x * u + self.view_y * v;
        Ray {
            origin: view_origin,
            dir: view_pos - view_origin,
        }
    }
}
