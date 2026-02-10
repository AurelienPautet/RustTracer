use crate::{ hittable::HitRecord, ray::Ray, vec3::{ Color, Vec3 } };

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Color);
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Color) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        (Ray::new(rec.p, scatter_direction), self.albedo)
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Color) {
        let reflected = r_in.direction().reflect(rec.normal);
        (Ray::new(rec.p, reflected), self.albedo)
    }
}
