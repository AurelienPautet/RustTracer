use crate::{
    Color,
    Point3,
    Size,
    bvh::BvhNode,
    camera::Camera,
    hittable::{ Hittable, HittableList },
    material::{ Dielectric, Lambertian, Material, Metal },
    random_f32,
    random_f32_range,
    sphere::Sphere,
};
use std::sync::Arc;

pub struct Scene {
    pub camera: Camera,
    pub world: Box<dyn Hittable>,
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
            world: Box::new(BvhNode::new(world.into_objects())),
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
            world: Box::new(BvhNode::new(world.into_objects())),
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
            world: Box::new(BvhNode::new(world.into_objects())),
        }
    }

    pub fn create_scene4(size: Size) -> Scene {
        let mut world = HittableList::new();

        let ground_material: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian {
            albedo: Color::new(0.3, 0.4, 0.3),
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

        for a in -20..20 {
            for b in -20..20 {
                let choose_mat = random_f32();
                let center = Point3::new(
                    (a as f32) + 0.9 * random_f32(),
                    0.15,
                    (b as f32) + 0.9 * random_f32()
                );

                let sphere_material: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.7 {
                    // Lambertian
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian { albedo });
                } else if choose_mat < 0.9 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f32_range(0.0, 0.3);
                    sphere_material = Arc::new(Metal { albedo, fuzziness: fuzz });
                } else {
                    // Glass
                    sphere_material = Arc::new(Dielectric {
                        refraction_index: random_f32_range(1.3, 2.0),
                        frostedness: random_f32_range(0.0, 0.08),
                    });
                }

                world.add(Box::new(Sphere::new(center, 0.15, sphere_material)));
            }
        }

        for i in 0..8 {
            let angle = ((i as f32) * std::f32::consts::TAU) / 8.0;
            let radius = 6.0;
            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            let stack_mat: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
                albedo: Color::new(0.8, 0.7 + (0.1 * (i as f32)) / 8.0, 0.6),
                fuzziness: 0.05,
            });

            for height in 0..5 {
                world.add(
                    Box::new(
                        Sphere::new(
                            Point3::new(x, 0.2 + (height as f32) * 0.4, z),
                            0.2,
                            stack_mat.clone()
                        )
                    )
                );
            }
        }

        let center_glass: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
            refraction_index: 1.5,
            frostedness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, center_glass.clone())));
        world.add(Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), -1.9, center_glass)));

        let corner_positions = [
            Point3::new(10.0, 1.5, 10.0),
            Point3::new(-10.0, 1.5, 10.0),
            Point3::new(10.0, 1.5, -10.0),
            Point3::new(-10.0, 1.5, -10.0),
        ];

        let corner_colors = [
            Color::new(0.9, 0.2, 0.2),
            Color::new(0.2, 0.9, 0.2),
            Color::new(0.2, 0.2, 0.9),
            Color::new(0.9, 0.9, 0.2),
        ];

        for (pos, color) in corner_positions.iter().zip(corner_colors.iter()) {
            let corner_mat: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
                albedo: *color,
                fuzziness: 0.0,
            });
            world.add(Box::new(Sphere::new(*pos, 1.5, corner_mat)));
        }

        let cam = Camera::new(20.0, size, 2000, 4);

        Scene {
            camera: cam,
            world: Box::new(BvhNode::new(world.into_objects())),
        }
    }

    pub fn create_scene5(size: Size) -> Scene {
        let mut world = HittableList::new();

        let core_radius = 2.0;
        let core_spheres = 80;
        for _ in 0..core_spheres {
            let theta = random_f32() * std::f32::consts::TAU;
            let phi = random_f32() * std::f32::consts::PI;
            let r = random_f32() * core_radius;

            let x = r * phi.sin() * theta.cos();
            let y = r * phi.sin() * theta.sin();
            let z = r * phi.cos();

            let choose_mat = random_f32();
            let sphere_material: Arc<dyn Material + Send + Sync>;

            if choose_mat < 0.3 {
                let warmth = random_f32_range(0.8, 1.0);
                sphere_material = Arc::new(Metal {
                    albedo: Color::new(warmth, warmth * 0.9, warmth * 0.6),
                    fuzziness: random_f32_range(0.0, 0.1),
                });
            } else if choose_mat < 0.6 {
                let hue = random_f32();
                let color = if hue < 0.33 {
                    Color::new(0.9, random_f32_range(0.3, 0.6), random_f32_range(0.2, 0.4))
                } else if hue < 0.66 {
                    Color::new(
                        random_f32_range(0.2, 0.4),
                        random_f32_range(0.5, 0.9),
                        random_f32_range(0.8, 1.0)
                    )
                } else {
                    Color::new(
                        random_f32_range(0.6, 0.9),
                        random_f32_range(0.3, 0.6),
                        random_f32_range(0.8, 1.0)
                    )
                };
                sphere_material = Arc::new(Lambertian { albedo: color });
            } else {
                sphere_material = Arc::new(Dielectric {
                    refraction_index: random_f32_range(1.3, 2.5),
                    frostedness: random_f32_range(0.0, 0.1),
                });
            }

            let size = random_f32_range(0.1, 0.3);
            world.add(Box::new(Sphere::new(Point3::new(x, y, z), size, sphere_material)));
        }

        let num_arms = 4;
        let arm_length = 15.0;
        let spheres_per_arm = 150;

        for arm in 0..num_arms {
            let arm_offset = ((arm as f32) * std::f32::consts::TAU) / (num_arms as f32);

            for i in 0..spheres_per_arm {
                let t = (i as f32) / (spheres_per_arm as f32);
                let radius = core_radius + t * arm_length;

                let spiral_tightness = 0.5;
                let angle =
                    arm_offset + t * std::f32::consts::TAU * 2.0 + radius * spiral_tightness;

                let x = angle.cos() * radius + random_f32_range(-0.5, 0.5);
                let z = angle.sin() * radius + random_f32_range(-0.5, 0.5);
                let y = random_f32_range(-0.3, 0.3) * (1.0 - t * 0.5);

                let choose_mat = random_f32();
                let sphere_material: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.4 {
                    let star_type = random_f32();
                    let color = if star_type < 0.2 {
                        Color::new(0.7, 0.8, 1.0)
                    } else if star_type < 0.4 {
                        Color::new(1.0, 0.5, 0.3)
                    } else if star_type < 0.6 {
                        Color::new(1.0, 0.95, 0.7)
                    } else if star_type < 0.8 {
                        Color::new(0.95, 0.95, 1.0)
                    } else {
                        Color::new(1.0, 0.7, 0.4)
                    };

                    sphere_material = Arc::new(Metal {
                        albedo: color,
                        fuzziness: random_f32_range(0.0, 0.15),
                    });
                } else if choose_mat < 0.75 {
                    let nebula_type = random_f32();
                    let color = if nebula_type < 0.33 {
                        Color::new(
                            random_f32_range(0.7, 1.0),
                            random_f32_range(0.2, 0.4),
                            random_f32_range(0.6, 0.9)
                        )
                    } else if nebula_type < 0.66 {
                        Color::new(
                            random_f32_range(0.2, 0.4),
                            random_f32_range(0.6, 0.9),
                            random_f32_range(0.8, 1.0)
                        )
                    } else {
                        Color::new(
                            random_f32_range(0.5, 0.8),
                            random_f32_range(0.2, 0.4),
                            random_f32_range(0.7, 1.0)
                        )
                    };

                    sphere_material = Arc::new(Lambertian { albedo: color });
                } else {
                    sphere_material = Arc::new(Dielectric {
                        refraction_index: random_f32_range(1.2, 2.0),
                        frostedness: random_f32_range(0.0, 0.15),
                    });
                }

                let size = random_f32_range(0.08, 0.25) * (1.0 - t * 0.4);
                world.add(Box::new(Sphere::new(Point3::new(x, y, z), size, sphere_material)));
            }
        }

        let central_star: Arc<dyn Material + Send + Sync> = Arc::new(Metal {
            albedo: Color::new(1.0, 0.95, 0.8),
            fuzziness: 0.0,
        });
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.5, central_star)));

        let prominent_stars = 20;
        for _ in 0..prominent_stars {
            let radius = random_f32_range(3.0, 12.0);
            let angle = random_f32() * std::f32::consts::TAU;
            let height = random_f32_range(-2.0, 2.0);

            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            let star_type = random_f32();
            let (color, mat_type) = if star_type < 0.3 {
                (Color::new(0.8, 0.9, 1.0), true)
            } else if star_type < 0.5 {
                (Color::new(1.0, 0.4, 0.2), true)
            } else if star_type < 0.7 {
                (Color::new(1.0, 1.0, 0.8), true)
            } else {
                (Color::new(0.9, 0.4, 0.9), false)
            };

            let material: Arc<dyn Material + Send + Sync> = if mat_type {
                Arc::new(Metal {
                    albedo: color,
                    fuzziness: random_f32_range(0.0, 0.05),
                })
            } else {
                Arc::new(Lambertian { albedo: color })
            };

            world.add(
                Box::new(
                    Sphere::new(Point3::new(x, height, z), random_f32_range(0.4, 0.8), material)
                )
            );
        }

        let exotic_objects = 30;
        for _ in 0..exotic_objects {
            let radius = random_f32_range(4.0, 14.0);
            let angle = random_f32() * std::f32::consts::TAU;
            let height = random_f32_range(-3.0, 3.0);

            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            let exotic_mat: Arc<dyn Material + Send + Sync> = Arc::new(Dielectric {
                refraction_index: random_f32_range(1.3, 2.5),
                frostedness: random_f32_range(0.0, 0.2),
            });

            world.add(
                Box::new(
                    Sphere::new(Point3::new(x, height, z), random_f32_range(0.3, 0.6), exotic_mat)
                )
            );
        }

        let halo_objects = 50;
        for _ in 0..halo_objects {
            let radius = random_f32_range(15.0, 25.0);
            let angle = random_f32() * std::f32::consts::TAU;
            let height = random_f32_range(-5.0, 5.0);

            let x = angle.cos() * radius;
            let z = angle.sin() * radius;

            let choose_mat = random_f32();
            let material: Arc<dyn Material + Send + Sync> = if choose_mat < 0.6 {
                Arc::new(Metal {
                    albedo: Color::new(
                        random_f32_range(0.6, 1.0),
                        random_f32_range(0.6, 1.0),
                        random_f32_range(0.6, 1.0)
                    ),
                    fuzziness: random_f32_range(0.0, 0.2),
                })
            } else {
                Arc::new(Lambertian {
                    albedo: Color::new(
                        random_f32_range(0.3, 0.7),
                        random_f32_range(0.3, 0.7),
                        random_f32_range(0.3, 0.7)
                    ),
                })
            };

            world.add(
                Box::new(
                    Sphere::new(Point3::new(x, height, z), random_f32_range(0.15, 0.35), material)
                )
            );
        }

        let cam = Camera::new(20.0, size, 2000, 4);

        Scene {
            camera: cam,
            world: Box::new(BvhNode::new(world.into_objects())),
        }
    }
}
