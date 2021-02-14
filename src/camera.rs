use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pos: Vec3,
    view_origin: Vec3,
    view_x: Vec3,
    view_y: Vec3,
}

pub struct Config {
    pub pos: Vec3,
    pub target: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aspect: f64,
}

impl Camera {
    pub fn new(config: Config) -> Camera {
        let view_height = 2.0 * (config.vfov / 2.0).tan();
        let view_width = view_height * config.aspect;

        let lookdir = (config.target - config.pos).unit();
        let view_right = Vec3::cross(lookdir, config.vup).unit();
        let view_up = Vec3::cross(view_right, lookdir);

        let pos = config.pos;
        let view_x = view_right * view_width;
        let view_y = view_up * view_height;
        let view_origin = pos - view_x / 2.0 - view_y / 2.0 + lookdir;

        Camera {
            pos,
            view_origin,
            view_x,
            view_y,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let view_pos = self.view_origin + self.view_x * u + self.view_y * v;
        Ray {
            origin: self.pos,
            dir: view_pos - self.pos,
        }
    }
}
