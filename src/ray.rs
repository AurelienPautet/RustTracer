use crate::vec3::{ Color, Point3, Vec3, dot };

pub struct Ray(Point3, Vec3);

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray(origin, direction)
    }

    pub fn origin(&self) -> Point3 {
        self.0
    }

    pub fn direction(&self) -> Vec3 {
        self.1
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + self.direction() * t
    }
}

pub fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let n = (ray.at(t) - Point3::new(0.0, 0.0, -1.0)).to_unit_vector();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = ray.direction().to_unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center - ray.origin();
    let a = dot(&ray.direction(), &ray.direction());
    let b = -2.0 * dot(&ray.direction(), &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-b - discriminant.sqrt()) / (2.0 * a)
}
