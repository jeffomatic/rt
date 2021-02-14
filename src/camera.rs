use glam::Vec3A;

use crate::{ray::Ray, util::random_within_unit_disk};

pub struct Camera {
    pos: Vec3A,
    view_origin: Vec3A,
    view_up: Vec3A,
    view_right: Vec3A,
    view_x: Vec3A, // view_right, scaled by focus distance and viewport width
    view_y: Vec3A, // view_up, scaled by focus distance and viewport height
    lens_radius: f32,
}

pub struct Config {
    pub pos: Vec3A,
    pub target: Vec3A,
    pub vup: Vec3A,
    pub vfov: f32,
    pub aspect: f32,
    pub lens_radius: f32, // half the aperture, which is a diameter
    pub focus_distance: f32,
}

impl Camera {
    pub fn new(config: Config) -> Camera {
        let view_height = 2.0 * (config.vfov / 2.0).tan();
        let view_width = view_height * config.aspect;

        let lookdir = (config.target - config.pos).normalize();
        let view_right = lookdir.cross(config.vup).normalize();
        let view_up = view_right.cross(lookdir);

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

    pub fn ray(&self, u: f32, v: f32) -> Ray {
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
