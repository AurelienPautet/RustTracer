use crate::{
    Color,
    Point3,
    Size,
    camera::Camera,
    hittable::HittableList,
    material::{ Dielectric, Lambertian, Material, Metal },
    random_f32,
    random_f32_range,
    sphere::Sphere,
};
use std::sync::Arc;

pub struct Scene {
    pub camera: Camera,
    pub world: HittableList,
}

impl Scene {
    pub fn create_scene1(size: Size) -> Scene {
        let mut world = HittableList::new();

        let ground_material: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

        for a in -6..6 {
            for b in -6..6 {
                let choose_mat = random_f32();
                let center = Point3::new(
                    (a as f32) + 0.9 * random_f32(),
                    0.2,
                    (b as f32) + 0.9 * random_f32()
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let sphere_material: Arc<dyn Material + Send + Sync>;

                    if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        sphere_material = Arc::new(Lambertian { albedo });
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = random_f32_range(0.0, 0.5);
                        sphere_material = Arc::new(Metal { albedo, fuzziness: fuzz });
                    } else {
                        sphere_material = Arc::new(Dielectric {
                            refraction_index: random_f32_range(0.5, 2.5),
                            frostedness: random_f32_range(0.0, 0.05),
                        });
                    }

                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }

        let material1: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 1.5,
            frostedness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

        let material2: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        });
        world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

        let material3: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzziness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

        let cam = Camera::new(20.0, size, 2000, 4);

        Scene {
            camera: cam,
            world,
        }
    }

    pub fn create_scene2(size: Size) -> Scene {
        let mut world = HittableList::new();

        let ground_material: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
            albedo: Color::new(0.2, 0.3, 0.4),
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

        let glass_outer: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 1.5,
            frostedness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, glass_outer.clone())));
        world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), -0.9, glass_outer)));

        let num_ring_spheres = 8;
        let ring_radius = 3.0;
        for i in 0..num_ring_spheres {
            let angle = ((i as f32) * std::f32::consts::TAU) / (num_ring_spheres as f32);
            let x = angle.cos() * ring_radius;
            let z = angle.sin() * ring_radius;

            let metal_color = Color::new(
                0.5 + 0.5 * ((i as f32) / (num_ring_spheres as f32)),
                0.7,
                0.5 + 0.5 * (1.0 - (i as f32) / (num_ring_spheres as f32))
            );
            let metal_mat: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
                albedo: metal_color,
                fuzziness: random_f32_range(0.0, 0.2),
            });
            world.add(Box::new(Sphere::new(Point3::new(x, 0.4, z), 0.4, metal_mat)));
        }

        let glass_high: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 2.4,
            frostedness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(-2.5, 0.7, -1.0), 0.7, glass_high)));

        let glass_mid: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 1.5,
            frostedness: 0.05,
        });
        world.add(Box::new(Sphere::new(Point3::new(2.5, 0.7, -1.0), 0.7, glass_mid)));

        let glass_low: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 1.1,
            frostedness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.7, 2.0), 0.7, glass_low)));

        let colors = [
            Color::new(0.8, 0.1, 0.1),
            Color::new(0.1, 0.8, 0.1),
            Color::new(0.1, 0.1, 0.8),
            Color::new(0.8, 0.8, 0.1),
            Color::new(0.8, 0.1, 0.8),
        ];

        for (i, color) in colors.iter().enumerate() {
            let angle = ((i as f32) * std::f32::consts::TAU) / (colors.len() as f32) + 0.5;
            let radius = 5.5;
            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            let lamb_mat: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
                albedo: *color,
            });
            world.add(Box::new(Sphere::new(Point3::new(x, 0.5, z), 0.5, lamb_mat)));
        }

        let mirror: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
            albedo: Color::new(0.95, 0.95, 0.95),
            fuzziness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, 2.0, -8.0), 2.0, mirror)));

        let cam = Camera::new(20.0, size, 2000, 4);

        Scene {
            camera: cam,
            world,
        }
    }

    pub fn create_scene3(size: Size) -> Scene {
        let mut world = HittableList::new();

        let ground_material: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
            albedo: Color::new(0.3, 0.3, 0.35),
            fuzziness: 0.4,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

        let tall_glass1: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 1.5,
            frostedness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(-3.0, 0.5, 0.0), 0.5, tall_glass1.clone())));
        world.add(Box::new(Sphere::new(Point3::new(-3.0, 1.5, 0.0), 0.5, tall_glass1.clone())));
        world.add(Box::new(Sphere::new(Point3::new(-3.0, 2.5, 0.0), 0.5, tall_glass1)));

        let tall_glass2: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 1.8,
            frostedness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(3.0, 0.5, 0.0), 0.5, tall_glass2.clone())));
        world.add(Box::new(Sphere::new(Point3::new(3.0, 1.5, 0.0), 0.5, tall_glass2.clone())));
        world.add(Box::new(Sphere::new(Point3::new(3.0, 2.5, 0.0), 0.5, tall_glass2)));

        let center_metal: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
            albedo: Color::new(0.9, 0.7, 0.3),
            fuzziness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, 1.5, 0.0), 1.5, center_metal)));

        let floating_spheres = vec![
            (Point3::new(-1.5, 3.5, -2.0), Color::new(0.9, 0.2, 0.2)),
            (Point3::new(1.5, 3.5, -2.0), Color::new(0.2, 0.9, 0.2)),
            (Point3::new(0.0, 4.5, -2.0), Color::new(0.2, 0.2, 0.9))
        ];

        for (pos, color) in floating_spheres {
            let lamb: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
                albedo: color,
            });
            world.add(Box::new(Sphere::new(pos, 0.5, lamb)));
        }

        let orbit_radius = 4.0;
        for i in 0..6 {
            let angle = ((i as f32) * std::f32::consts::TAU) / 6.0;
            let x = angle.cos() * orbit_radius;
            let z = angle.sin() * orbit_radius;

            let mat: Arc<dyn Material + Send + Sync> = if i % 2 == 0 {
                Arc::new(Metal {
                    albedo: Color::new(0.8, 0.8, 0.9),
                    fuzziness: 0.1,
                })
            } else {
                Arc::new(Dielectric {
                    refraction_index: 2.0,
                    frostedness: 0.02,
                })
            };
            world.add(Box::new(Sphere::new(Point3::new(x, 0.3, z), 0.3, mat)));
        }

        let large_frosted: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 1.5,
            frostedness: 0.15,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.8, -6.0), 1.5, large_frosted)));

        let cam = Camera::new(20.0, size, 2000, 4);

        Scene {
            camera: cam,
            world,
        }
    }
}
