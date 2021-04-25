use super::*;

mod intersections {
    use super::*;
    use crate::Normal3D;
    use std::f64::consts::PI;

    #[test]
    fn intersecting_a_ray_with_the_default_world_should_produce_a_sorted_list_of_intersections() {
        let world = World::default();
        let intersections = world.intersect(&Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Normal3D::POSITIVE_Z,
        ));

        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections.get(0).unwrap().t, 4.0);
        assert_eq!(intersections.get(1).unwrap().t, 4.5);
        assert_eq!(intersections.get(2).unwrap().t, 5.5);
        assert_eq!(intersections.get(3).unwrap().t, 6.0);
    }

    #[test]
    fn the_colour_should_be_black_when_a_ray_hits_nothing() {
        let world = World::default();
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Y);

        assert_eq!(world.colour_at(ray), Colour::BLACK);
    }

    #[test]
    fn a_hit_on_an_opaque_object_should_not_include_the_colour_of_objects_behind_it() {
        let mut world = World::empty();
        {
            let front = Object::plane()
                .with_material(Material {
                    ambient: 1.0,
                    specular: 0.0,
                    diffuse: 0.0,
                    pattern: Pattern::solid(Colour::new(0.1, 0.1, 0.1)),
                    transparency: 0.0,
                    ..Default::default()
                })
                .transformed(Transform::identity().rotate_x(-PI / 2.0));
            world.objects.push(front);
        };

        {
            let back = Object::sphere()
                .with_material(Material {
                    pattern: Pattern::solid(Colour::GREEN),
                    ambient: 1.0,
                    diffuse: 0.0,
                    specular: 0.0,
                    ..Default::default()
                })
                .transformed(Transform::identity().translate_z(1.0));
            world.objects.push(back);
        };

        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::ORIGIN));

        let ray = Ray::new(Point3D::new(0.0, 0.0, -1.0), Normal3D::POSITIVE_Z);
        assert_eq!(world.colour_at(ray), Colour::new(0.1, 0.1, 0.1));
    }
}

mod shading {
    use super::*;
    use crate::{Camera, Normal3D};
    use std::f64::consts::PI;

    #[test]
    fn should_correctly_shade_an_external_hit() {
        let world = World::default();
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let colour = world.colour_at(ray);
        let expected = Colour::new(
            0.38066119308103435,
            0.47582649135129296,
            0.28549589481077575,
        );

        assert!(
            approx_eq!(Colour, colour, expected, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            colour
        );
    }

    #[test]
    fn should_correctly_shade_an_internal_hit() {
        let mut world = World::default();
        world.lights = vec![Light::point(Colour::WHITE, Point3D::new(0.0, 0.25, 0.0))];

        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Normal3D::POSITIVE_Z);
        let sphere = world
            .objects
            .get(1)
            .expect("Default world should have objects");

        let intersections = sphere.intersect(&ray);
        let intersection = intersections.hit(None);
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        let hit_data = HitData::from(&ray, intersection, intersections);
        let colour = world.shade_hit(&hit_data);
        let expected = Colour::new(0.9049844720832575, 0.9049844720832575, 0.9049844720832575);

        assert!(
            approx_eq!(Colour, colour, expected, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            colour,
            expected
        );
    }

    #[test]
    fn the_colour_should_be_the_shaded_surface_when_the_ray_hits_an_object() {
        let world = World::default();
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        assert!(approx_eq!(
            Colour,
            world.colour_at(ray),
            Colour::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            ),
            epsilon = f32::EPSILON as f64
        ));
    }

    #[test]
    fn the_colour_should_not_include_any_objects_behind_the_ray() {
        let mut world = World::default();
        world
            .objects
            .iter_mut()
            .for_each(|obj| obj.material.ambient = 1.0);

        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.75), Normal3D::NEGATIVE_Z);

        assert_eq!(world.colour_at(ray), Colour::WHITE);
    }

    #[test]
    fn a_hit_on_a_face_exposed_by_a_csg_subtraction_should_be_lit_as_external() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(2.0, 7.0, -4.0)));

        let csg = Object::csg_difference(
            Object::cube().with_material(Material {
                pattern: Pattern::solid(Colour::new(0.9, 0.9, 0.0)),
                ..Default::default()
            }),
            Object::sphere()
                .transformed(
                    Transform::identity()
                        .translate_x(0.5)
                        .translate_y(0.5)
                        .translate_z(-0.5),
                )
                .with_material(Material {
                    pattern: Pattern::solid(Colour::WHITE),
                    ambient: 0.01,
                    ..Default::default()
                }),
        )
        .transformed(Transform::identity().translate_y(1.0).rotate_y(PI / 4.0));

        world.objects.push(csg);

        let ray = Ray::new(Point3D::new(0.0, 2.0, -6.0), Normal3D::POSITIVE_Z);
        let actual = world.colour_at(ray);
        let expected = Colour::new(0.1557279290614545, 0.1557279290614545, 0.1557279290614545);
        // note: if the ray misses, the colour will be black;
        // if the ray hits, but the colour is shaded incorrectly, the colour will be very dark;
        // if the ray hits and the colour is shaded correctly, the colour will be light
        assert!(
            approx_eq!(Colour, expected, actual, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            actual
        );
    }

    // using the hit `point` rather than `over_point` results in hideous acne at certain angles;
    // this test ensures that doesn't happen
    #[test]
    fn a_checker_pattern_on_a_rotated_plane_should_not_have_noise() {
        use nonzero_ext::*;

        let mut world = World::empty();
        world.lights.push(Light::point(
            Colour::WHITE,
            Point3D::new(-10.0, 10.0, -10.0),
        ));
        world.objects.push(
            Object::plane()
                .transformed(Transform::identity().rotate_x(-PI / 4.0))
                .with_material(Material {
                    pattern: Pattern::checkers(Colour::WHITE, Colour::BLACK),
                    // ensure lighting doesn't affect colours
                    ambient: 1.0,
                    specular: 0.0,
                    diffuse: 0.0,
                    ..Default::default()
                }),
        );

        // it's easier to use the camera to generate the rays
        let camera = Camera::new(
            nonzero!(400u16),
            nonzero!(400u16),
            PI / 3.0,
            Transform::view_transform(
                Point3D::new(0.0, 1.0, -5.0),
                Point3D::new(0.0, 0.0, 0.0),
                Normal3D::POSITIVE_Y,
            ),
        );

        let rays = (0..5)
            .into_iter()
            .flat_map(|x| (0..5).into_iter().map(move |y| (x, y)))
            .map(|(x, y)| camera.ray_at(x, y, 0.5, 0.5))
            .collect::<Vec<_>>();

        rays.into_iter()
            .for_each(|ray| assert_eq!(world.colour_at(ray), Colour::WHITE))
    }

    #[test]
    fn setting_a_group_material_should_override_the_material_of_its_children() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(-2.0, 0.0, 0.0)));

        let sphere = Object::sphere().with_material(Material {
            pattern: Pattern::solid(Colour::BLUE),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        });
        let group = Object::group(vec![sphere]).with_material(Material {
            pattern: Pattern::solid(Colour::GREEN),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        });

        world.add(group);

        let colour = world.colour_at(Ray::new(Point3D::new(-1.0, 0.0, 0.0), Normal3D::POSITIVE_X));
        assert_eq!(colour, Colour::GREEN);
    }

    #[test]
    fn a_capped_cylinder_should_not_have_acne() {
        let mut world = World::empty();
        world.lights.push(Light::point(
            Colour::WHITE,
            Point3D::new(-10.0, 100.0, -100.0),
        ));

        let pedestal = Object::cylinder()
            .min_y(-0.15)
            .max_y(0.0)
            .capped()
            .build()
            .transformed(Transform::identity().scale_x(30.0).scale_z(30.0))
            .with_material(Material {
                pattern: Pattern::solid(Colour::RED),
                ambient: 0.0,
                specular: 0.0,
                diffuse: 1.0,
                ..Default::default()
            });

        world.add(pedestal);

        let camera = Camera::new(
            nonzero_ext::nonzero!(1920u16),
            nonzero_ext::nonzero!(1080u16),
            1.2,
            Transform::view_transform(
                Point3D::new(0.0, 2.5, -10.0),
                Point3D::new(0.0, 1.0, 0.0),
                Normal3D::POSITIVE_Y,
            ),
        );

        let expected = Colour::new(0.6957377128787232, 0.0, 0.0);

        let colour = world.colour_at(camera.ray_at(62, 592, 0.5, 0.5));
        assert!(
            approx_eq!(Colour, colour, expected, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            colour
        );

        let expected = Colour::new(0.6957372881303316, 0.0, 0.0);
        let colour = world.colour_at(camera.ray_at(63, 592, 0.5, 0.5));
        assert!(
            approx_eq!(Colour, colour, expected, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            colour
        );

        let expected = Colour::new(0.6957368602150138, 0.0, 0.0);
        let colour = world.colour_at(camera.ray_at(64, 592, 0.5, 0.5));
        assert!(
            approx_eq!(Colour, colour, expected, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            colour
        );
    }

    #[test]
    fn overlapping_objects_should_not_have_acne() {
        let mut world = World::empty();

        world.lights.push(Light::point(
            Colour::WHITE,
            Point3D::new(-10.0, 100.0, -100.0),
        ));

        let pedestal = Object::cylinder()
            .min_y(-0.15)
            .max_y(0.0)
            .capped()
            .build()
            .with_material(Material {
                pattern: Pattern::solid(Colour::RED),
                ambient: 0.0,
                specular: 0.0,
                diffuse: 1.0,
                ..Default::default()
            });
        world.add(pedestal);

        let glass_box = Object::cube()
            .with_material(Material {
                pattern: Pattern::solid(Colour::greyscale(0.1)),
                casts_shadow: false,
                ambient: 0.0,
                diffuse: 1.0,
                specular: 0.0,
                transparency: 0.9,
                refractive: 1.0,
                ..Default::default()
            })
            .transformed(
                Transform::identity()
                    .scale_x(0.5002689)
                    .scale_y(0.346323)
                    .scale_z(0.2181922)
                    .translate_x(-0.0338752)
                    .translate_y(0.346323)
                    .translate_z(0.0598042),
            );

        world.add(glass_box);

        let camera = Camera::new(
            nonzero_ext::nonzero!(1920u16),
            nonzero_ext::nonzero!(1080u16),
            1.2,
            Transform::view_transform(
                Point3D::new(0.0, 2.5, -10.0),
                Point3D::new(0.0, 1.0, 0.0),
                Normal3D::POSITIVE_Y,
            ),
        );

        // let expected = Colour::new(0.7050813513879224, 0.07052098992794008, 0.07052098992794008);
        // let actual = world.colour_at(camera.ray_at(889, 669, 0.5, 0.5));
        //
        // assert!(
        //     approx_eq!(Colour, expected, actual, epsilon = f32::EPSILON as f64),
        //     "{:?} != {:?}",
        //     expected,
        //     actual
        // );

        let expected = Colour::new(0.7050813513879224, 0.07052098992794008, 0.07052098992794008);
        let actual = world.colour_at(camera.ray_at(890, 669, 0.5, 0.5));

        assert!(
            approx_eq!(Colour, expected, actual, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            actual
        );

        let expected = Colour::new(0.7050813513879224, 0.07052098992794008, 0.07052098992794008);
        let actual = world.colour_at(camera.ray_at(891, 669, 0.5, 0.5));

        assert!(
            approx_eq!(Colour, expected, actual, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            actual
        );
    }
}

mod lighting {
    use super::*;
    use crate::Normal3D;

    #[test]
    fn lighting_a_point_in_shadow_should_only_have_ambient() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0)));
        // target object
        world.objects.push(Object::sphere());
        // object in-between target/ray and light source
        world.objects.push(
            Object::sphere()
                .transformed(Transform::identity().translate_z(-7.5))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::BLUE),
                    ..Default::default()
                }),
        );

        let colour = world.colour_at(Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z));

        assert_eq!(colour, Colour::greyscale(0.1));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface_should_only_have_ambient() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, 10.0)));
        world.objects.push(Object::sphere());

        let colour = world.colour_at(Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z));

        assert_eq!(colour, Colour::greyscale(0.1));
    }
}

mod reflection_and_refraction {
    use super::*;
    use crate::{Normal3D, Vector3D};
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn a_hit_on_a_reflective_surface_should_combine_the_surface_colour_with_the_reflected_colour() {
        let mut world = World::default();
        {
            let reflective_plane = Object::plane()
                .with_material(Material {
                    reflective: 0.5,
                    ..Default::default()
                })
                .transformed(Transform::identity().translate_y(-1.0));

            world.objects.push(reflective_plane);
        };

        assert!(approx_eq!(
            Colour,
            world.colour_at(Ray::new(
                Point3D::new(0.0, 0.0, -3.0),
                Vector3D::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0).normalised(),
            )),
            Colour::new(0.8767560060717737, 0.9243386603443418, 0.8291733517992057),
            epsilon = f32::EPSILON as f64
        ));
    }

    #[test]
    fn a_hit_facing_a_pair_of_parallel_mirrors_should_not_reflect_infinitely() {
        let reflective_non_blinding_material = Material {
            reflective: 1.0,
            ambient: 0.2,
            specular: 0.0,
            diffuse: 0.0,
            ..Default::default()
        };

        let mut world = World::empty();
        {
            let upper = Object::plane()
                .with_material(reflective_non_blinding_material.clone())
                .transformed(Transform::identity().rotate_x(PI).translate_y(1.0));
            world.objects.push(upper);
        };

        {
            let lower = Object::plane()
                .with_material(reflective_non_blinding_material)
                .transformed(Transform::identity().translate_y(-1.0));
            world.objects.push(lower);
        };
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, 0.0)));

        assert!(approx_eq!(
            Colour,
            world.colour_at(Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Y)),
            Colour::WHITE,
            epsilon = f32::EPSILON as f64
        ));
    }

    #[test]
    fn a_hit_on_a_refractive_object_should_include_the_colour_from_refracted_rays() {
        let mut world = World::default();
        {
            let refractive_plane = Object::plane()
                .transformed(Transform::identity().translate_y(-1.0))
                .with_material(Material {
                    transparency: 0.5,
                    refractive: 1.5,
                    ..Default::default()
                });

            world.objects.push(refractive_plane);
        };

        {
            let ball = Object::sphere()
                .transformed(Transform::identity().translate_y(-3.5).translate_z(-0.5))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::RED),
                    ambient: 0.5,
                    ..Default::default()
                });

            world.objects.push(ball);
        };

        let ray = Ray::new(
            Point3D::new(0.0, 0.0, -3.0),
            Vector3D::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0).normalised(),
        );

        let expected = world.colour_at(ray);
        let actual = Colour::new(1.1128630897687959, 0.686425385324466, 0.686425385324466);

        assert!(
            approx_eq!(Colour, expected, actual, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            actual
        );
    }
}

mod transparency {
    use super::*;
    use crate::{Camera, Normal3D, Vector3D};
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn a_hit_on_a_fully_transparent_non_refractive_object_should_include_the_colour_of_objects_behind_it(
    ) {
        let mut world = World::empty();
        {
            let front = Object::plane()
                .with_material(Material {
                    ambient: 1.0,
                    specular: 0.0,
                    diffuse: 0.0,
                    pattern: Pattern::solid(Colour::BLACK),
                    transparency: 1.0,
                    refractive: 1.0,
                    ..Default::default()
                })
                .transformed(Transform::identity().rotate_x(-PI / 2.0));
            world.objects.push(front);
        };

        {
            let back = Object::sphere()
                .with_material(Material {
                    pattern: Pattern::solid(Colour::GREEN),
                    ambient: 1.0,
                    diffuse: 0.0,
                    specular: 0.0,
                    ..Default::default()
                })
                .transformed(Transform::identity().translate_z(1.0));
            world.objects.push(back);
        };

        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, 0.5)));

        let ray = Ray::new(Point3D::new(0.0, 0.0, -1.0), Normal3D::POSITIVE_Z);
        assert_eq!(world.colour_at(ray), Colour::GREEN);
    }
    #[test]
    fn a_hit_on_a_transparent_refractive_and_reflective_object_should_have_fresnel() {
        let mut world = World::default();
        {
            let refractive_plane = Object::plane()
                .transformed(Transform::identity().translate_y(-1.0))
                .with_material(Material {
                    transparency: 0.5,
                    reflective: 0.5,
                    refractive: 1.5,
                    ..Default::default()
                });

            world.objects.push(refractive_plane);
        };

        {
            let ball = Object::sphere()
                .transformed(Transform::identity().translate_y(-3.5).translate_z(-0.5))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::RED),
                    ambient: 0.5,
                    ..Default::default()
                });

            world.objects.push(ball);
        };

        let ray = Ray::new(
            Point3D::new(0.0, 0.0, -3.0),
            Vector3D::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0).normalised(),
        );

        let expected = world.colour_at(ray);
        let actual = Colour::new(1.1029302361646764, 0.6964342236428152, 0.6924306883154755);

        assert!(
            approx_eq!(Colour, expected, actual, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            actual
        );
    }

    #[test]
    fn a_transparent_sphere_should_include_the_colour_of_objects_behind_it() {
        let mut world = World::empty();
        world.lights.push(Light::point(
            Colour::WHITE,
            Point3D::new(-10.0, 10.0, -10.0),
        ));

        {
            let wall = Object::plane()
                .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(5.0))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::BLUE),
                    ambient: 1.0,
                    ..Default::default()
                });

            world.objects.push(wall);
        };

        {
            let glass_sphere = Object::sphere()
                .transformed(Transform::identity().translate_z(1.0))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::new(0.05, 0.05, 0.05)),
                    transparency: 1.0,
                    ..Default::default()
                });

            world.objects.push(glass_sphere);
        };

        let ray = Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Z);

        let expected = Colour::new(0.06315462059737657, 0.06315462059737657, 1.7180008057464167);
        let actual = world.colour_at(ray);

        assert!(
            approx_eq!(Colour, expected, actual, epsilon = f32::EPSILON as f64),
            "{:?} != {:?}",
            expected,
            actual
        );
    }

    #[test]
    fn a_fully_transparent_material_with_shadow_ray_collisions_disabled_should_not_cast_shadows() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -2.0)));
        world.objects.push(
            Object::plane()
                .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(2.0))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::WHITE),
                    // ensure the material colour should be 100% white iff light reaches it
                    ambient: 0.0,
                    diffuse: 1.0,
                    specular: 0.0,
                    ..Default::default()
                }),
        );
        world.objects.push(
            Object::plane()
                .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(1.0))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::BLACK),
                    // prevent light reflections from transparent plane
                    specular: 0.0,
                    transparency: 1.0,
                    casts_shadow: false,
                    ..Default::default()
                }),
        );

        let ray = Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Z);
        assert_eq!(world.colour_at(ray), Colour::WHITE);
    }

    #[test]
    fn a_fully_transparent_material_should_not_cast_shadows() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -2.0)));
        world.objects.push(
            Object::plane()
                .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(2.0))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::WHITE),
                    // ensure the material colour should be 100% white iff light reaches it
                    ambient: 0.0,
                    diffuse: 1.0,
                    specular: 0.0,
                    ..Default::default()
                }),
        );
        world.objects.push(
            Object::plane()
                .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(1.0))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::BLACK),
                    // prevent light reflections from transparent plane
                    specular: 0.0,
                    transparency: 1.0,
                    // 100% of light should get through anyway
                    casts_shadow: true,
                    ..Default::default()
                }),
        );

        let ray = Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Z);
        assert_eq!(world.colour_at(ray), Colour::WHITE);
    }

    #[test]
    fn a_half_transparent_material_should_allow_half_of_the_light_source_through() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -2.0)));
        world.objects.push(
            Object::plane()
                .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(2.0))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::WHITE),
                    // ensure the material colour should be 100% white iff light reaches it
                    ambient: 0.0,
                    diffuse: 1.0,
                    specular: 0.0,
                    ..Default::default()
                }),
        );
        world.objects.push(
            Object::plane()
                .transformed(Transform::identity().rotate_x(-PI / 2.0).translate_z(1.0))
                .with_material(Material {
                    pattern: Pattern::solid(Colour::BLACK),
                    // prevent light reflections from semi-transparent plane
                    specular: 0.0,
                    transparency: 0.5,
                    // 50% of light should get through anyway
                    casts_shadow: true,
                    ..Default::default()
                }),
        );

        let ray = Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Z);
        // 50% of the light reaches the surface, and only 50% of the _reflected_ light passes back
        // through the plane into the camera; therefore only 25% of the surface colour reaches the camera
        assert_eq!(world.colour_at(ray), Colour::greyscale(0.25));
    }

    #[test]
    fn a_partially_transparent_red_material_should_cast_red_tinted_shadows() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(-6.0, 15.0, -8.0)));

        let floor = Object::plane().with_material(Material {
            pattern: Pattern::solid(Colour::WHITE),
            diffuse: 0.9,
            // have to crank the ambient up so it actually appears white rather than grey
            ambient: 0.35,
            ..Default::default()
        });

        world.objects.push(floor);

        let red_pane = Object::cube()
            .transformed(
                Transform::identity()
                    .scale_z(0.01)
                    .translate_z(4.5)
                    .translate_y(3.0),
            )
            .with_material(Material {
                pattern: Pattern::solid(Colour::new(0.33, 0.0, 0.0)),
                transparency: 0.9,
                ..Default::default()
            });

        world.objects.push(red_pane);

        // use camera to calculate ray angle
        let camera = Camera::new(
            nonzero_ext::nonzero!(800u16),
            nonzero_ext::nonzero!(600u16),
            PI / 3.0,
            Transform::view_transform(
                Point3D::new(0.0, 4.0, -3.0),
                Point3D::new(0.0, 3.5, 0.0),
                Normal3D::POSITIVE_Y,
            ),
        );
        let ray = camera.ray_at(550, 450, 0.5, 0.5);

        let slightly_red = Colour::new(0.8358840839713972, 0.7392412057309325, 0.7392412057309325);
        assert_eq!(world.colour_at(ray), slightly_red);
    }

    #[test]
    fn a_partially_transparent_yellow_material_should_cast_yellow_tinted_shadows() {
        let mut world = World::empty();
        world
            .lights
            .push(Light::point(Colour::WHITE, Point3D::new(-6.0, 15.0, -8.0)));

        let floor = Object::plane().with_material(Material {
            pattern: Pattern::solid(Colour::WHITE),
            diffuse: 0.9,
            // have to crank the ambient up so it actually appears white rather than grey
            ambient: 0.35,
            ..Default::default()
        });

        world.objects.push(floor);

        let yellow_pane = Object::cube()
            .transformed(
                Transform::identity()
                    .scale_z(0.01)
                    .translate_z(4.5)
                    .translate_y(3.0),
            )
            .with_material(Material {
                pattern: Pattern::solid(Colour::new(0.0, 0.33, 0.33)),
                transparency: 0.9,
                ..Default::default()
            });

        world.objects.push(yellow_pane);

        // use camera to calculate ray angle
        let camera = Camera::new(
            nonzero_ext::nonzero!(800u16),
            nonzero_ext::nonzero!(600u16),
            PI / 3.0,
            Transform::view_transform(
                Point3D::new(0.0, 4.0, -3.0),
                Point3D::new(0.0, 3.5, 0.0),
                Normal3D::POSITIVE_Y,
            ),
        );
        let ray = camera.ray_at(550, 450, 0.5, 0.5);

        let slightly_yellow =
            Colour::new(0.7392412057309325, 0.7875626448511649, 0.7875626448511649);
        assert_eq!(world.colour_at(ray), slightly_yellow);
    }
}
