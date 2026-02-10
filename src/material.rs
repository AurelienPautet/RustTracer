use crate::{ hittable::HitRecord, ray::Ray, vec3::{ Color, Vec3, dot } };

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((Ray::new(rec.p, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = r_in.direction().reflect(rec.normal);
        reflected = reflected.to_unit_vector() + self.fuzziness * Vec3::random_unit_vector();
        let scattered: Ray = Ray::new(rec.p, reflected);
        if dot(&scattered.direction(), &rec.normal) <= 0.0 {
            return None;
        }
        Some((scattered, self.albedo))
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction().to_unit_vector();
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction: Vec3;

        if ri * sin_theta > 1.0 {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, ri);
        }

        let scattered: Ray = Ray::new(rec.p, direction);

        Some((scattered, Color::new(1.0, 1.0, 1.0)))
    }
}
