use super::*;

mod unit_tests {
    use super::*;
    use crate::{Camera, Normal3D, Vector3D};
    use std::f64::consts::{PI, SQRT_2};

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
    fn should_correctly_shade_an_external_hit() {
        let world = World::default();
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);
        let sphere = world
            .objects
            .first()
            .expect("Default world should have objects");
        let intersections = sphere.intersect(&ray);
        let intersection = intersections.hit();
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        let hit_data = HitData::from(&ray, intersection, intersections);
        let colour = world.shade_hit(&hit_data);

        assert!(approx_eq!(
            Colour,
            colour,
            Colour::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            ),
            epsilon = f32::EPSILON as f64
        ));
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
        let intersection = intersections.hit();
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
    fn the_colour_should_be_black_when_a_ray_hits_nothing() {
        let world = World::default();
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Y);

        assert_eq!(world.colour_at(ray), Colour::BLACK);
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
    fn a_point_with_no_objects_collinear_to_the_light_should_not_be_shadowed() {
        let world = World::default();
        let light = world
            .lights
            .get(0)
            .expect("The default world should have a light");

        assert!(!world.is_in_shadow(Point3D::new(0.0, 10.0, 0.0), light))
    }

    #[test]
    fn a_point_behind_a_lit_object_should_be_shadowed() {
        let world = World::default();
        let light = world
            .lights
            .get(0)
            .expect("The default world should have a light");

        assert!(world.is_in_shadow(Point3D::new(10.0, -10.0, 10.0), light))
    }

    #[test]
    fn a_point_behind_the_light_should_not_be_shadowed() {
        let world = World::default();
        let light = world
            .lights
            .get(0)
            .expect("The default world should have a light");

        assert!(!world.is_in_shadow(Point3D::new(-20.0, 20.0, -20.0), light))
    }

    #[test]
    fn a_point_in_between_the_light_and_an_object_should_not_be_shadowed() {
        let world = World::default();
        let light = world
            .lights
            .get(0)
            .expect("The default world should have a light");

        assert!(!world.is_in_shadow(Point3D::new(-2.0, 2.0, -2.0), light))
    }

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

        assert!(approx_eq!(
            Colour,
            world.colour_at(ray),
            Colour::new(0.9364253889815014, 0.6864253889815014, 0.6864253889815014),
            epsilon = f32::EPSILON as f64
        ));
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

        assert!(approx_eq!(
            Colour,
            world.colour_at(ray),
            Colour::new(0.9339151414147093, 0.6964342273743777, 0.6924306920172272),
            epsilon = f32::EPSILON as f64
        ));
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

        assert!(approx_eq!(
            Colour,
            world.colour_at(ray),
            Colour::new(0.03598076211353316, 0.03598076211353316, 1.690826949711632),
            epsilon = f32::EPSILON as f64
        ));
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
            .map(|(x, y)| camera.ray_at(x, y))
            .collect::<Vec<_>>();

        rays.into_iter()
            .for_each(|ray| assert_eq!(world.colour_at(ray), Colour::WHITE))
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
        assert_eq!(world.colour_at(ray), Colour::greyscale(0.5));
    }
}
