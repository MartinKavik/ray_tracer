use super::vec::{Vec3, Point3, Color};
use super::hit::{Hit, World};

pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn color(r: &Self, world: &World, depth: u64) -> Color {
        if depth <= 0 {
            // If we've exceeded the ray bounce limit, no more light is gathered
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                attenuation * Self::color(&scattered, world, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_direction = r.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
