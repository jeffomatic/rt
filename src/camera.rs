use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pos: Vec3,
    view_origin: Vec3,
    view_x: Vec3,
    view_y: Vec3,
}

impl Camera {
    pub fn new(aspect: f64) -> Camera {
        let view_height = 2.0;
        let view_width = view_height * aspect;
        let flength = 1.0;

        let pos = Vec3::new(0.0, 0.0, 0.0);
        let view_x = Vec3::new(view_width as f64, 0.0, 0.0);
        let view_y = Vec3::new(0.0, view_height, 0.0);
        let view_origin = pos - view_x / 2.0 - view_y / 2.0 + Vec3::new(0.0, 0.0, -flength);

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
