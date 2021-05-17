use super::*;

mod shape_tests {
    use super::*;
    use crate::Pattern;
    use std::f64::consts::PI;

    #[test]
    fn lighting_with_the_eye_in_between_the_light_and_the_surface_should_have_full_intensity() {
        let sphere = Object::sphere();
        let point = Point3D::new(0.0, 0.0, -1.0);

        let normal = sphere.normal_at(point, None);
        let eye_vector = Normal3D::NEGATIVE_Z;
        let light = Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));

        let lit_material = sphere.colour_at(point, light.colour(), eye_vector, normal, &light);
        assert_eq!(lit_material, Colour::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_at_a_45_degree_angle_to_the_surface_normal_should_have_no_specular() {
        let sphere = Object::sphere();
        let point = Point3D::new(0.0, 0.0, -1.0);

        let normal = sphere.normal_at(point, None);
        let eye_vector =
            Vector3D::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0).normalised();
        let light = Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));

        let lit_material = sphere.colour_at(point, light.colour(), eye_vector, normal, &light);
        assert_eq!(lit_material, Colour::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_light_at_a_45_degree_angle_to_the_surface_normal_should_have_no_specular_and_less_diffuse(
    ) {
        let sphere = Object::sphere().transformed(Transform::identity().translate_z(1.0));
        let point = Point3D::new(0.0, 0.0, 0.0);

        let normal = sphere.normal_at(point, None);
        let eye_vector = Normal3D::NEGATIVE_Z;
        let light = Light::point(Colour::WHITE, Point3D::new(0.0, 10.0, -10.0));

        let lit_material = sphere.colour_at(point, light.colour(), eye_vector, normal, &light);
        assert_eq!(
            lit_material,
            Colour::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );
    }

    #[test]
    fn lighting_with_the_light_at_45_deg_and_the_eye_at_neg_45_deg_to_the_surface_normal_should_have_less_diffuse(
    ) {
        let sphere = Object::sphere().transformed(Transform::identity().translate_z(1.0));
        let point = Point3D::new(0.0, 0.0, 0.0);

        let normal = sphere.normal_at(point, None);
        let eye_vector =
            Vector3D::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0).normalised();
        let light = Light::point(Colour::WHITE, Point3D::new(0.0, 10.0, -10.0));

        let lit_material = sphere.colour_at(point, light.colour(), eye_vector, normal, &light);
        assert_eq!(
            lit_material,
            Colour::new(1.6363961030679328, 1.6363961030679328, 1.6363961030679328)
        );
    }

    #[test]
    fn translating_an_object_should_translate_the_pattern_in_world_space() {
        let sphere = Object::sphere()
            .transformed(Transform::identity().translate_x(1.0))
            .with_material(Material {
                kind: MaterialKind::Pattern(Pattern::striped(Colour::WHITE, Colour::BLACK)),
                ambient: 1.0,
                diffuse: 0.0,
                specular: 0.0,
                ..Default::default()
            });
        let point = Point3D::new(0.5, 0.0, 0.0);

        let normal = sphere.normal_at(point, None);

        assert_eq!(
            sphere.colour_at(
                point,
                Colour::WHITE,
                Normal3D::NEGATIVE_X,
                normal,
                &Light::point(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0))
            ),
            Colour::BLACK
        );
    }

    #[test]
    fn rotating_an_object_should_rotate_the_pattern_in_world_space() {
        let sphere = Object::sphere()
            .transformed(Transform::identity().rotate_y(PI))
            .with_material(Material {
                kind: MaterialKind::Pattern(Pattern::striped(Colour::WHITE, Colour::BLACK)),
                ambient: 1.0,
                diffuse: 0.0,
                specular: 0.0,
                ..Default::default()
            });

        let point = Point3D::new(-0.5, 0.0, 0.0);
        let normal = sphere.normal_at(point, None);

        assert_eq!(
            sphere.colour_at(
                point,
                Colour::WHITE,
                Normal3D::NEGATIVE_X,
                normal,
                &Light::point(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0))
            ),
            Colour::WHITE
        );
    }
}

mod sphere_tests {
    use super::*;
    use crate::Pattern;

    #[test]
    fn should_be_able_to_calculate_the_normal_on_the_x_axis() {
        let sphere = Object::sphere();
        let normal = sphere.normal_at(Point3D::new(1.0, 0.0, 0.0), None);
        assert_eq!(normal, Normal3D::POSITIVE_X);
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_on_the_y_axis() {
        let sphere = Object::sphere();
        let normal = sphere.normal_at(Point3D::new(0.0, 1.0, 0.0), None);
        assert_eq!(normal, Normal3D::POSITIVE_Y);
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_on_the_z_axis() {
        let sphere = Object::sphere();
        let normal = sphere.normal_at(Point3D::new(0.0, 0.0, 1.0), None);
        assert_eq!(normal, Normal3D::POSITIVE_Z);
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_at_an_arbitrary_point_on_a_sphere() {
        let sphere = Object::sphere();
        let normal = sphere.normal_at(
            Point3D::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
            None,
        );
        assert_eq!(
            normal,
            Vector3D::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            )
            .normalised()
        );
    }

    #[test]
    fn should_be_able_to_calculate_a_surface_normal_on_a_translated_sphere() {
        use std::f64::consts::FRAC_1_SQRT_2;

        let sphere = Object::sphere().transformed(Transform::identity().translate_y(1.0));

        let normal = sphere.normal_at(Point3D::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2), None);
        assert!(approx_eq!(
            Normal3D,
            normal,
            Vector3D::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2).normalised()
        ));
    }

    #[test]
    fn should_be_able_to_calculate_a_surface_normal_on_a_transformed_sphere() {
        use std::f64::consts::PI;

        let transform = Transform::identity()
            .rotate_z(PI / 5.0)
            .scale_x(1.0)
            .scale_y(0.5)
            .scale_z(1.0);
        let sphere = Object::sphere().transformed(transform);

        let normal = sphere.normal_at(
            Point3D::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
            None,
        );
        assert_eq!(
            normal,
            Vector3D::new(0.0, 0.9701425001453319, -0.24253562503633294).normalised()
        );
    }

    #[test]
    fn a_ray_passing_through_the_world_origin_should_intersect_a_unit_sphere_at_two_points() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_on_a_tangent_with_a_unit_sphere_should_intersect_twice_at_the_same_point() {
        let ray = Ray::new(Point3D::new(0.0, 1.0, -5.0), Normal3D::POSITIVE_Z);
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 5.0);
        assert_eq!(intersections.underlying()[1].t, 5.0);
    }

    #[test]
    fn a_ray_passing_over_a_unit_sphere_should_not_intersect() {
        let ray = Ray::new(Point3D::new(0.0, 2.0, -5.0), Normal3D::POSITIVE_Z);
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert!(intersections.underlying().is_empty());
    }

    #[test]
    fn a_ray_originating_inside_a_unit_sphere_should_intersect_in_positive_and_negative_time() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Normal3D::POSITIVE_Z);
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, -1.0);
        assert_eq!(intersections.underlying()[1].t, 1.0);
    }

    #[test]
    fn a_ray_originating_outside_a_sphere_and_pointing_away_from_it_should_intersect_twice_in_negative_time(
    ) {
        let ray = Ray::new(Point3D::new(0.0, 0.0, 5.0), Normal3D::POSITIVE_Z);
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, -6.0);
        assert_eq!(intersections.underlying()[1].t, -4.0);
    }

    #[test]
    fn a_ray_should_intersect_a_scaled_sphere() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);
        let sphere = Object::sphere().transformed(Transform::identity().scale_all(2.0));

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 3.0);
        assert_eq!(intersections.underlying()[1].t, 7.0);
    }

    #[test]
    fn a_ray_should_not_intersect_a_sphere_translated_away_from_it() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);
        let translation = Transform::identity().translate_x(5.0);
        let sphere = Object::sphere().transformed(translation);

        let intersections = sphere.intersect(&ray);
        assert!(intersections.underlying().is_empty())
    }

    #[test]
    fn lighting_a_point_on_the_left_hemisphere_of_a_default_sphere_with_a_default_stripe_pattern_should_use_the_secondary_colour(
    ) {
        let sphere = Object::sphere().with_material(Material {
            kind: MaterialKind::Pattern(Pattern::striped(Colour::WHITE, Colour::BLACK)),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        });

        let point = Point3D::new(-0.5, 0.0, 0.0);
        let normal = sphere.normal_at(point, None);

        assert_eq!(
            sphere.colour_at(
                point,
                Colour::WHITE,
                Normal3D::NEGATIVE_X,
                normal,
                &Light::point(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0)),
            ),
            Colour::BLACK
        );
    }

    #[test]
    fn lighting_a_point_on_the_right_hemisphere_of_a_default_sphere_with_a_default_stripe_pattern_should_use_the_primary_colour(
    ) {
        let sphere = Object::sphere().with_material(Material {
            kind: MaterialKind::Pattern(Pattern::striped(Colour::WHITE, Colour::BLACK)),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        });

        let point = Point3D::new(0.5, 0.0, 0.0);
        let normal = sphere.normal_at(point, None);

        assert_eq!(
            sphere.colour_at(
                point,
                Colour::WHITE,
                Normal3D::NEGATIVE_X,
                normal,
                &Light::point(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0)),
            ),
            Colour::WHITE
        );
    }
}

mod plane_tests {
    use super::*;
    use proptest::prelude::*;
    use std::f64::consts::PI;

    proptest! {
        #[test]
        fn the_normal_of_an_xz_plane_is_constant_at_all_points(x in crate::util::reasonable_f64(), z in crate::util::reasonable_f64()) {
            assert_eq!(
                Object::plane().normal_at(Point3D::new(x, 0.0, z), None),
                Normal3D::POSITIVE_Y
            );
        }

        #[test]
        fn the_normal_of_an_xy_plane_is_constant_at_all_points(x in crate::util::reasonable_f64(), y in crate::util::reasonable_f64()) {
            let plane = Object::plane().transformed(Transform::identity().rotate_x(PI / 2.0));

            assert!(approx_eq!(
                Normal3D,
                plane.normal_at(Point3D::new(x, y, 0.0), None),
                Normal3D::POSITIVE_Z
            ));
        }

        #[test]
        fn the_normal_of_a_yz_plane_is_constant_at_all_points(y in crate::util::reasonable_f64(), z in crate::util::reasonable_f64()) {
            let plane = Object::plane().transformed(Transform::identity().rotate_z(PI / 2.0));

            assert!(approx_eq!(
                Normal3D,
                plane.normal_at(Point3D::new(0.0, y, z), None),
                Normal3D::NEGATIVE_X
            ));
        }
    }

    #[test]
    fn a_plane_is_not_intersected_by_a_parallel_ray() {
        assert!(Object::plane()
            .intersect(&Ray::new(Point3D::new(0.0, 1.0, 0.0), Normal3D::POSITIVE_X))
            .is_empty());
    }

    #[test]
    fn a_plane_is_not_intersected_by_a_coplanar_ray() {
        assert!(Object::plane()
            .intersect(&Ray::new(Point3D::new(0.0, 0.0, 0.0), Normal3D::POSITIVE_X))
            .is_empty());
    }

    #[test]
    fn a_plane_is_intersected_by_a_ray_originating_from_above() {
        let plane = Object::plane();
        let intersections =
            plane.intersect(&Ray::new(Point3D::new(0.0, 1.0, 0.0), Normal3D::NEGATIVE_Y));

        assert_eq!(intersections.len(), 1);

        assert_eq!(intersections.underlying()[0].t, 1.0);
    }

    #[test]
    fn a_plane_is_intersected_by_a_ray_originating_from_below() {
        let plane = Object::plane();
        let intersections = plane.intersect(&Ray::new(
            Point3D::new(0.0, -1.0, 0.0),
            Normal3D::POSITIVE_Y,
        ));

        assert_eq!(intersections.len(), 1);

        assert_eq!(intersections.underlying()[0].t, 1.0);
    }
}

mod cube_tests {
    use super::*;

    #[test]
    fn a_ray_directly_towards_the_pos_x_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(5.0, 0.5, 0.0), Normal3D::NEGATIVE_X);

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_neg_x_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(-5.0, 0.5, 0.0), Normal3D::POSITIVE_X);

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_pos_y_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, 5.0, 0.0), Normal3D::NEGATIVE_Y);

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_neg_y_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, -5.0, 0.0), Normal3D::POSITIVE_Y);

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_pos_z_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, 0.0, 5.0), Normal3D::NEGATIVE_Z);

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_neg_z_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_starting_inside_the_cube_should_intersect_in_positive_and_negative_t() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, 0.0, 0.0), Normal3D::POSITIVE_Z);

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, -1.0);
        assert_eq!(intersections.underlying()[1].t, 1.0);
    }

    #[test]
    fn an_ray_passing_diagonally_by_the_cube_should_not_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(
            Point3D::new(-2.0, 0.0, 0.0),
            Vector3D::new(0.2673, 0.5345, 0.8018).normalised(),
        );

        assert!(cube.intersect(&ray).is_empty());
    }

    #[test]
    fn an_ray_parallel_to_the_pos_x_face_originating_from_the_right_should_not_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(2.0, 2.0, 0.0), Normal3D::NEGATIVE_X);

        assert!(cube.intersect(&ray).is_empty());
    }

    #[rustfmt::skip]
    #[test]
    fn the_normal_of_a_cube_point_should_be_based_off_the_largest_component() {
        vec![
            (Point3D::new(1.0, 0.5, -0.8), Normal3D::POSITIVE_X),
            (Point3D::new(-1.0, -0.2, 0.9), Normal3D::NEGATIVE_X),
            (Point3D::new(-0.4, 1.0, -0.1), Normal3D::POSITIVE_Y),
            (Point3D::new(0.3, -1.0, -0.7), Normal3D::NEGATIVE_Y),
            (Point3D::new(-0.6, 0.3, 1.0), Normal3D::POSITIVE_Z),
            (Point3D::new(0.4, 0.4, -1.0), Normal3D::NEGATIVE_Z),
            (Point3D::new(1.0, 1.0, 1.0), Normal3D::POSITIVE_X),
            (Point3D::new(-1.0, -1.0, -1.0), Normal3D::NEGATIVE_X),
        ]
        .into_iter()
        .for_each(|(point, normal)| {
            assert_eq!(Object::cube().normal_at(point, None), normal);
        })
    }
}

mod cylinder_tests {
    use super::*;

    #[test]
    fn a_ray_that_misses_an_infinite_cylinder_should_not_intersect() {
        let cylinder = Object::cylinder().build();

        vec![
            Ray::new(Point3D::new(1.0, 0.0, 0.0), Normal3D::POSITIVE_Y),
            Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Y),
            Ray::new(
                Point3D::new(0.0, 0.0, -5.0),
                Vector3D::new(1.0, 1.0, 1.0).normalised(),
            ),
        ]
        .into_iter()
        .for_each(|ray| assert_eq!(cylinder.intersect(&ray).len(), 0))
    }

    #[test]
    fn a_ray_that_hits_an_infinite_cylinder_should_intersect_twice() {
        let cylinder = Object::cylinder().build();

        vec![
            (
                Ray::new(Point3D::new(1.0, 0.0, -5.0), Normal3D::POSITIVE_Z),
                5.0,
                5.0,
                "tangent",
            ),
            (
                Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z),
                4.0,
                6.0,
                "through centre",
            ),
            (
                Ray::new(
                    Point3D::new(0.5, 0.0, -5.0),
                    Vector3D::new(0.1, 1.0, 1.0).normalised(),
                ),
                6.80798191702732,
                7.088723439378861,
                "from angle",
            ),
        ]
        .into_iter()
        .for_each(|(ray, t0, t1, scenario)| {
            let intersections = cylinder.intersect(&ray);

            assert_eq!(intersections.len(), 2, "{}", scenario);
            assert_eq!(intersections.underlying()[0].t, t0, "{}", scenario);
            assert_eq!(intersections.underlying()[1].t, t1, "{}", scenario);
        })
    }

    #[test]
    fn the_normal_of_an_infinite_cylinder_should_have_0_y() {
        let cylinder = Object::cylinder().build();

        vec![
            (Point3D::new(1.0, 0.0, 0.0), Normal3D::POSITIVE_X),
            (Point3D::new(0.0, 5.0, -1.0), Normal3D::NEGATIVE_Z),
            (Point3D::new(0.0, -2.0, 1.0), Normal3D::POSITIVE_Z),
            (Point3D::new(-1.0, 1.0, 0.0), Normal3D::NEGATIVE_X),
        ]
        .into_iter()
        .for_each(|(point, normal)| {
            assert_eq!(cylinder.normal_at(point, None), normal);
        })
    }

    #[test]
    fn rays_that_miss_a_finite_hollow_cylinder_should_not_intersect() {
        let cylinder = Object::cylinder().min_y(1.0).max_y(2.0).build();

        vec![
            (
                "starts inside cylinder; escapes without hitting sides",
                Point3D::new(0.0, 1.5, 0.0),
                Vector3D::new(0.1, 1.0, 0.0),
            ),
            (
                "perpendicular ray passing above",
                Point3D::new(0.0, 3.0, -5.0),
                Vector3D::new(0.0, 0.0, 1.0),
            ),
            (
                "perpendicular ray passing below",
                Point3D::new(0.0, 0.0, -5.0),
                Vector3D::new(0.0, 0.0, 1.0),
            ),
            (
                "perpendicular ray passing above (max is exclusive)",
                Point3D::new(0.0, 2.0, -5.0),
                Vector3D::new(0.0, 0.0, 1.0),
            ),
            (
                "perpendicular ray passing below (min is exclusive)",
                Point3D::new(0.0, 1.0, -5.0),
                Vector3D::new(0.0, 0.0, 1.0),
            ),
        ]
        .into_iter()
        .for_each(|(scenario, origin, direction)| {
            let ray = Ray::new(origin, direction.normalised());

            assert_eq!(cylinder.intersect(&ray).len(), 0, "{}", scenario);
        })
    }

    #[test]
    fn a_ray_that_passes_through_a_hollow_finite_cylinder_intersects_twice() {
        let cylinder = Object::cylinder().min_y(1.0).max_y(2.0).build();

        let ray = Ray::new(Point3D::new(0.0, 1.5, -2.0), Normal3D::POSITIVE_Z);
        let intersections = cylinder.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.underlying()[0].t, 1.0);
        assert_eq!(intersections.underlying()[1].t, 3.0);
    }

    #[test]
    fn a_ray_passing_through_the_caps_of_a_capped_cylinder_should_intersect_twice() {
        let cylinder = Object::cylinder().min_y(1.0).max_y(2.0).capped().build();

        vec![
            (
                "passes through both caps from above",
                Point3D::new(0.0, 3.0, 0.0),
                Vector3D::new(0.0, -1.0, 0.0),
            ),
            (
                "diagonally intersects one cap and wall from above",
                Point3D::new(0.0, 3.0, -2.0),
                Vector3D::new(0.0, -1.0, 2.0),
            ),
            (
                "diagonally intersects one cap and wall from below",
                Point3D::new(0.0, 0.0, -2.0),
                Vector3D::new(0.0, 1.0, 2.0),
            ),
            (
                "diagonally intersects top cap and bottom 'corner'",
                Point3D::new(0.0, 4.0, -2.0),
                Vector3D::new(0.0, -1.0, 1.0),
            ),
            (
                "diagonally intersects bottom cap and top 'corner'",
                Point3D::new(0.0, -1.0, -2.0),
                Vector3D::new(0.0, 1.0, 1.0),
            ),
        ]
        .into_iter()
        .for_each(|(scenario, origin, direction)| {
            let ray = Ray::new(origin, direction.normalised());

            assert_eq!(cylinder.intersect(&ray).len(), 2, "{}", scenario);
        })
    }

    #[test]
    fn the_normal_vector_on_a_cap_should_either_be_pos_y_axis_or_neg_y_axis() {
        let cylinder = Object::cylinder().min_y(1.0).max_y(2.0).capped().build();

        vec![
            (Point3D::new(0.0, 1.0, 0.0), Normal3D::NEGATIVE_Y),
            (Point3D::new(0.5, 1.0, 0.0), Normal3D::NEGATIVE_Y),
            (Point3D::new(0.0, 1.0, 0.5), Normal3D::NEGATIVE_Y),
            (Point3D::new(0.0, 2.0, 0.0), Normal3D::POSITIVE_Y),
            (Point3D::new(0.5, 2.0, 0.0), Normal3D::POSITIVE_Y),
            (Point3D::new(0.0, 2.0, 0.5), Normal3D::POSITIVE_Y),
        ]
        .into_iter()
        .for_each(|(point, normal)| {
            assert_eq!(cylinder.normal_at(point, None), normal);
        })
    }
}

mod cone_tests {
    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn a_ray_that_passes_through_a_double_napped_cone_should_intersect_twice() {
        let cone = Object::cone().build();

        vec![
            (
                "Through middle",
                Point3D::new(0.0, 0.0, -5.0),
                Vector3D::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                "Through middle from angle",
                Point3D::new(0.0, 0.0, -5.0),
                Vector3D::new(1.0, 1.0, 1.0),
                8.660254037844386,
                8.660254037844386,
            ),
            (
                "Enters and leaves cone",
                Point3D::new(1.0, 1.0, -5.0),
                Vector3D::new(-0.5, -1.0, 1.0),
                4.550055679356349,
                49.449944320643645,
            ),
        ]
        .into_iter()
        .for_each(|(scenario, origin, direction, first, second)| {
            let ray = Ray::new(origin, direction.normalised());
            let intersections = cone.intersect(&ray);

            assert_eq!(intersections.len(), 2, "{}", scenario);
            assert_eq!(intersections.underlying()[0].t, first, "{}", scenario);
            assert_eq!(intersections.underlying()[1].t, second, "{}", scenario);
        })
    }

    #[test]
    fn a_ray_parallel_to_one_half_of_a_double_napped_cone_should_intersect_once() {
        let cone = Object::cone().build();

        let ray = Ray::new(
            Point3D::new(0.0, 0.0, -1.0),
            Vector3D::new(0.0, 1.0, 1.0).normalised(),
        );
        let intersections = cone.intersect(&ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections.underlying()[0].t, 0.3535533905932738);
    }

    #[test]
    fn a_ray_should_be_able_to_intersect_the_caps_of_a_capped_cone() {
        let cone = Object::cone().min_y(-0.5).max_y(0.5).capped().build();

        vec![
            (
                "Misses cone",
                Point3D::new(0.0, 0.0, -5.0),
                Vector3D::new(0.0, 1.0, 0.0),
                0,
            ),
            (
                "Through cap and out side",
                Point3D::new(0.0, 0.0, -0.25),
                Vector3D::new(0.0, 1.0, 1.0),
                2,
            ),
            (
                "Through both caps and both cones",
                Point3D::new(0.0, 0.0, -0.25),
                Vector3D::new(0.0, 1.0, 0.0),
                4,
            ),
        ]
        .into_iter()
        .for_each(|(scenario, origin, direction, expected)| {
            let ray = Ray::new(origin, direction.normalised());

            let intersections = cone.intersect(&ray);
            assert_eq!(intersections.len(), expected, "{}", scenario);
        })
    }

    #[test]
    #[rustfmt::skip]
    fn should_be_able_to_calculate_the_normal_of_any_point_on_a_double_napped_cone() {
        let cone = Object::cone().build();

        vec![
            ("Middle point", Point3D::ORIGIN, Vector3D::new(0.0, 0.0, 0.0).normalised()),
            ("Positive y", Point3D::new(1.0, 1.0, 1.0), Vector3D::new(1.0, -SQRT_2, 1.0).normalised()),
            ("Negative y", Point3D::new(-1.0, -1.0, 0.0), Vector3D::new(-1.0, 1.0, 0.0).normalised()),
        ]
        .into_iter()
        .for_each(|(scenario, point, normal)| {
            assert!(approx_eq!(Normal3D, cone.normal_at(point, None), normal), "{}", scenario);
        })
    }
}

mod group_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn a_ray_should_not_intersect_an_empty_group() {
        let group = Object::group(vec![]);
        let ray = Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Z);

        assert!(group.intersect(&ray).is_empty());
    }

    #[test]
    fn a_ray_should_intersect_all_children_in_a_non_empty_group_in_the_path_of_the_ray() {
        let first = Object::sphere();
        let first_id = first.id();

        let second = Object::sphere().transformed(Transform::identity().translate_z(-3.0));
        let second_id = second.id();

        let group = Object::group(vec![
            first,
            second,
            Object::sphere().transformed(Transform::identity().translate_x(5.0)),
        ]);
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = group.intersect(&ray);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections.underlying()[0].with.id(), second_id);
        assert_eq!(intersections.underlying()[1].with.id(), second_id);
        assert_eq!(intersections.underlying()[2].with.id(), first_id);
        assert_eq!(intersections.underlying()[3].with.id(), first_id);
    }

    #[test]
    fn a_ray_should_intersect_the_children_of_a_transformed_group() {
        let group = Object::group(vec![
            Object::sphere().transformed(Transform::identity().translate_x(5.0))
        ])
        .transformed(Transform::identity().scale_all(2.0));

        let ray = Ray::new(Point3D::new(10.0, 0.0, -10.0), Normal3D::POSITIVE_Z);
        let intersections = group.intersect(&ray);
        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn group_transforms_should_apply_to_child_normals() {
        let object_transform = Transform::identity().translate_x(5.0);
        let inner_group_transform = Transform::identity().scale_x(1.0).scale_y(2.0).scale_z(3.0);
        let outer_group_transform = Transform::identity().rotate_y(PI / 2.0);

        let group = Object::group(vec![Object::group(vec![
            Object::sphere().transformed(object_transform)
        ])
        .transformed(inner_group_transform)])
        .transformed(outer_group_transform);

        // rust makes getting the reference back to the child sphere awkward, and the book doesn't explain where the point comes from
        // (otherwise it'd be easier to cast a ray to get an Intersection with the sphere)
        let sphere_ref = group
            .children()
            .first()
            .unwrap()
            .children()
            .first()
            .unwrap();

        assert!(approx_eq!(
            Normal3D,
            sphere_ref.normal_at(Point3D::new(1.7321, 1.1547, -5.5774), None),
            Vector3D::new(0.28570368184140726, 0.428543151781141, -0.8571605294481017).normalised()
        ));
    }
}

mod triangle_tests {
    use super::*;

    #[test]
    fn the_normal_of_a_triangle_should_be_constant() {
        let triangle = Object::triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );

        let normal = Normal3D::NEGATIVE_Z;

        assert_eq!(
            triangle.normal_at(Point3D::new(0.0, 0.5, 0.0), None),
            normal
        );
        assert_eq!(
            triangle.normal_at(Point3D::new(-0.5, 0.75, 0.0), None),
            normal
        );
        assert_eq!(
            triangle.normal_at(Point3D::new(0.5, 0.25, 0.0), None),
            normal
        );
    }

    #[test]
    fn a_ray_parallel_to_a_triangle_should_not_intersect() {
        let triangle = Object::triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );

        let ray = Ray::new(Point3D::new(0.0, -1.0, -2.0), Normal3D::POSITIVE_Y);

        assert!(triangle.intersect(&ray).is_empty())
    }

    #[test]
    fn a_ray_outside_the_p1_p3_edge_should_not_intersect() {
        let triangle = Object::triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );

        let ray = Ray::new(Point3D::new(1.0, -1.0, -2.0), Normal3D::POSITIVE_Z);

        assert!(triangle.intersect(&ray).is_empty())
    }

    #[test]
    fn a_ray_outside_the_p1_p2_edge_should_not_intersect() {
        let triangle = Object::triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );

        let ray = Ray::new(Point3D::new(-1.0, 1.0, -2.0), Normal3D::POSITIVE_Z);

        assert!(triangle.intersect(&ray).is_empty())
    }

    #[test]
    fn a_ray_outside_the_p2_p3_edge_should_not_intersect() {
        let triangle = Object::triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );

        let ray = Ray::new(Point3D::new(0.0, -1.0, -2.0), Normal3D::POSITIVE_Z);

        assert!(triangle.intersect(&ray).is_empty())
    }

    #[test]
    fn a_ray_inside_the_edges_of_a_triangle_should_intersect_once() {
        let triangle = Object::triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );

        let ray = Ray::new(Point3D::new(0.0, 0.5, -2.0), Normal3D::POSITIVE_Z);

        let intersections = triangle.intersect(&ray);
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections.underlying()[0].t, 2.0);
    }
}

mod smooth_triangles {
    use super::*;

    #[test]
    fn intersecting_a_smooth_triangle_should_populate_uv() {
        let triangle = Object::smooth_triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Normal3D::POSITIVE_Y,
            Normal3D::NEGATIVE_X,
            Normal3D::POSITIVE_X,
        );
        let ray = Ray::new(Point3D::new(-0.2, 0.3, -2.0), Normal3D::POSITIVE_Z);

        assert_eq!(
            triangle.intersect(&ray).underlying()[0].uv,
            Some((0.44999999999999996, 0.24999999999999997))
        );
    }

    #[test]
    fn the_normal_of_a_smooth_triangle_should_be_based_off_the_uv_of_the_intersection() {
        let triangle = Object::smooth_triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Normal3D::POSITIVE_Y,
            Normal3D::NEGATIVE_X,
            Normal3D::POSITIVE_X,
        );
        let ray = Ray::new(Point3D::new(-0.2, 0.3, -2.0), Normal3D::POSITIVE_Z);
        let uv = triangle.intersect(&ray).underlying()[0].uv;
        assert!(uv.is_some());
        let (u, v) = uv.unwrap();

        assert_eq!(
            // Point has no effect on normal as u,v is used instead
            triangle.normal_at(Point3D::ORIGIN, Some((u, v))),
            Vector3D::new(-0.554700196225229, 0.8320502943378437, 0.0).normalised()
        );
    }
}

mod constructive_solid_geometry {
    use super::*;

    #[test]
    fn a_ray_that_misses_both_objects_in_a_csg_should_not_intersect() {
        let csg = Object::csg_union(Object::sphere(), Object::cube());
        let ray = Ray::new(Point3D::new(0.0, 2.0, -5.0), Normal3D::POSITIVE_Z);

        assert!(csg.intersect(&ray).is_empty());
    }

    #[test]
    fn a_ray_that_intersects_overlapping_objects_in_a_csg_union_should_intersect_at_the_edge_of_each_object(
    ) {
        let left = Object::sphere();
        let right = Object::sphere().transformed(Transform::identity().translate_z(0.5));

        let left_id = left.id();
        let right_id = right.id();

        let csg = Object::csg_union(left, right);
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = csg.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[0].with.id, left_id);

        assert_eq!(intersections.underlying()[1].t, 6.5);
        assert_eq!(intersections.underlying()[1].with.id, right_id);
    }

    #[test]
    fn a_ray_that_intersects_overlapping_objects_in_a_csg_intersection_should_intersect_at_the_edges_of_the_overlap(
    ) {
        let left = Object::sphere();
        let right = Object::sphere().transformed(Transform::identity().translate_z(0.5));

        let left_id = left.id();
        let right_id = right.id();

        let csg = Object::csg_intersection(left, right);
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = csg.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.5);
        assert_eq!(intersections.underlying()[0].with.id, right_id);

        assert_eq!(intersections.underlying()[1].t, 6.0);
        assert_eq!(intersections.underlying()[1].with.id, left_id);
    }

    #[test]
    fn a_ray_that_intersects_overlapping_objects_in_a_csg_subtraction_should_intersect_exclusively_inside_the_left_object(
    ) {
        let left = Object::sphere();
        let right = Object::sphere().transformed(Transform::identity().translate_z(0.5));

        let left_id = left.id();
        let right_id = right.id();

        let csg = Object::csg_difference(left, right);
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = csg.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[0].with.id, left_id);

        assert_eq!(intersections.underlying()[1].t, 4.5);
        assert_eq!(intersections.underlying()[1].with.id, right_id);
    }

    // a naive implementation would compare intersection IDs with the IDs of its direct children, but this wouldn't work with Groups and other CSGs
    // this test ensures the implementation isn't that naive
    #[test]
    fn a_csg_comprising_groups_should_correctly_detect_intersections_on_the_children_of_children() {
        let first = Object::sphere().transformed(Transform::identity().translate_z(-3.0));

        let second = Object::sphere().transformed(Transform::identity().translate_z(-0.75));
        let second_id = second.id();

        let third = Object::sphere();
        let third_id = third.id();

        let fourth = Object::sphere().transformed(Transform::identity().translate_z(1.5));

        let csg = Object::csg_intersection(
            Object::group(vec![first, second]),
            Object::csg_difference(third, fourth),
        );
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = csg.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[0].with.id, third_id);

        assert_eq!(intersections.underlying()[1].t, 5.25);
        assert_eq!(intersections.underlying()[1].with.id, second_id);
    }

    #[test]
    fn transforming_a_csg_should_transform_the_children() {
        let first = Object::sphere().transformed(Transform::identity().translate_x(5.0));
        let first_id = first.id;

        let group = Object::csg_union(first, Object::sphere())
            .transformed(Transform::identity().scale_all(2.0));

        let ray = Ray::new(Point3D::new(10.0, 0.0, -10.0), Normal3D::POSITIVE_Z);
        let intersections = group.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.underlying()[0].with.id, first_id);
    }
}

mod bounding_boxes {
    use super::*;

    #[test]
    fn bounding_box_of_untransformed_primitives() {
        vec![
            (
                "sphere",
                Object::sphere(),
                BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0)),
            ),
            (
                "plane",
                Object::plane(),
                BoundingBox::new(
                    Point3D::new(-f64::MAX, 0.0, -f64::MAX),
                    Point3D::new(f64::MAX, 0.0, f64::MAX),
                ),
            ),
            (
                "cube",
                Object::cube(),
                BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0)),
            ),
            (
                "infinite cylinder",
                Object::cylinder().build(),
                BoundingBox::new(
                    Point3D::new(-1.0, -f64::MAX, -1.0),
                    Point3D::new(1.0, f64::MAX, 1.0),
                ),
            ),
            (
                "truncated cylinder",
                Object::cylinder().min_y(-5.0).max_y(3.0).build(),
                BoundingBox::new(Point3D::new(-1.0, -5.0, -1.0), Point3D::new(1.0, 3.0, 1.0)),
            ),
            (
                "infinite cone",
                Object::cone().build(),
                BoundingBox::new(
                    Point3D::new(-f64::MAX, -f64::MAX, -f64::MAX),
                    Point3D::new(f64::MAX, f64::MAX, f64::MAX),
                ),
            ),
            (
                "truncated cone",
                Object::cone().min_y(-5.0).max_y(3.0).build(),
                BoundingBox::new(Point3D::new(-5.0, -5.0, -5.0), Point3D::new(5.0, 3.0, 5.0)),
            ),
            (
                "triangle",
                Object::triangle(
                    Point3D::new(-3.0, 7.0, 2.0),
                    Point3D::new(6.0, 2.0, -4.0),
                    Point3D::new(2.0, -1.0, -1.0),
                ),
                BoundingBox::new(Point3D::new(-3.0, -1.0, -4.0), Point3D::new(6.0, 7.0, 2.0)),
            ),
        ]
        .into_iter()
        .for_each(|(scenario, object, bounds)| {
            assert_eq!(object.bounds, bounds, "{}", scenario);
        })
    }

    #[test]
    fn the_bounding_box_of_a_transformed_primitive_should_be_transformed() {
        let shape = Object::sphere().transformed(
            Transform::identity()
                .scale_x(0.5)
                .scale_y(2.0)
                .scale_z(4.0)
                .translate_x(1.0)
                .translate_y(-3.0)
                .translate_z(5.0),
        );

        assert_eq!(shape.bounds.min, Point3D::new(0.5, -5.0, 1.0));
        assert_eq!(shape.bounds.max, Point3D::new(1.5, -1.0, 9.0));
    }

    #[test]
    fn the_bounding_box_of_a_group_containing_transformed_children_should_contain_the_children_in_world_space(
    ) {
        let group = Object::group(vec![
            Object::sphere().transformed(
                Transform::identity()
                    .scale_all(2.0)
                    .translate_x(2.0)
                    .translate_y(5.0)
                    .translate_z(-3.0),
            ),
            Object::cylinder()
                .max_y(2.0)
                .min_y(-2.0)
                .build()
                .transformed(
                    Transform::identity()
                        .scale_x(0.5)
                        .scale_z(0.5)
                        .translate_x(-4.0)
                        .translate_y(-1.0)
                        .translate_z(4.0),
                ),
        ]);

        assert_eq!(group.bounds.min, Point3D::new(-4.5, -3.0, -5.0));
        assert_eq!(group.bounds.max, Point3D::new(4.0, 7.0, 4.5));
    }

    #[test]
    fn the_bounding_box_of_a_csg_should_be_large_enough_to_contain_its_children() {
        let csg = Object::csg_difference(
            Object::sphere(),
            Object::sphere().transformed(
                Transform::identity()
                    .translate_x(2.0)
                    .translate_y(3.0)
                    .translate_z(4.0),
            ),
        );

        assert_eq!(csg.bounds.min, Point3D::new(-1.0, -1.0, -1.0));
        assert_eq!(csg.bounds.max, Point3D::new(3.0, 4.0, 5.0));
    }
}

mod optimising_groups {
    use super::*;

    #[test]
    fn optimising_a_group_to_contain_as_few_children_as_possible_should_partition_the_children_based_on_the_split_bounding_box(
    ) {
        let left_sphere = Object::sphere().transformed(Transform::identity().translate_x(-2.0));
        let left_id = left_sphere.id;
        let left_bounds = left_sphere.bounds;

        let right_sphere = Object::sphere().transformed(Transform::identity().translate_x(2.0));
        let right_id = right_sphere.id;
        let right_bounds = right_sphere.bounds;

        let middle_sphere = Object::sphere();
        let middle_id = middle_sphere.id;

        let outer = Object::group(vec![left_sphere, right_sphere, middle_sphere]);
        let outer_bounds = outer.bounds;

        let optimised = outer.optimised(1);

        // ensure original bounds are unchanged
        assert_eq!(optimised.bounds, outer_bounds);

        assert_eq!(optimised.children().len(), 3);
        // ensure middle sphere is a direct child of the new group (as it doesn't entirely fit in either half of the bounds)
        assert_eq!(optimised.children()[0].id(), middle_id);

        // left sphere fits in left half of outer bounds
        assert_eq!(optimised.children()[1].children().len(), 1);
        assert_eq!(optimised.children()[1].children()[0].id(), left_id);
        // ensure the bounds of the sub-group are the same as (i.e. fully contain) the left sphere
        assert_eq!(optimised.children()[1].bounds, left_bounds);

        // right sphere fits in right half of outer bounds
        assert_eq!(optimised.children()[2].children().len(), 1);
        assert_eq!(optimised.children()[2].children()[0].id(), right_id);
        // ensure the bounds of the sub-group are the same as (i.e. fully contain) the right sphere
        assert_eq!(optimised.children()[2].bounds, right_bounds);
    }

    #[test]
    fn optimising_a_group_should_optimise_its_subgroups() {
        let s1 = Object::sphere().transformed(Transform::identity().translate_x(-2.0));
        let s1_id = s1.id;
        let s1_bounds = s1.bounds;

        let s2 =
            Object::sphere().transformed(Transform::identity().translate_x(2.0).translate_y(1.0));
        let s2_id = s2.id;
        let s2_bounds = s2.bounds;

        let s3 =
            Object::sphere().transformed(Transform::identity().translate_x(2.0).translate_y(-1.0));
        let s3_id = s3.id;
        let s3_bounds = s3.bounds;

        let s4 = Object::sphere();
        let s4_id = s4.id;

        let group = Object::group(vec![s4, Object::group(vec![s1, s2, s3])]);
        let optimised = group.optimised(3);

        assert_eq!(optimised.children().len(), 2);
        assert_eq!(optimised.children()[0].id, s4_id);

        let subgroup = &optimised.children()[1];
        assert_eq!(subgroup.children().len(), 2);

        assert_eq!(subgroup.children()[0].children().len(), 1);
        assert_eq!(subgroup.children()[0].children()[0].id, s1_id);
        assert_eq!(subgroup.children()[0].children()[0].bounds, s1_bounds);

        assert_eq!(subgroup.children()[1].children().len(), 2);
        assert_eq!(subgroup.children()[1].children()[0].id, s2_id);
        assert_eq!(subgroup.children()[1].children()[1].id, s3_id);

        assert!(subgroup.children()[1].bounds.fully_contains(&s2_bounds));
        assert!(subgroup.children()[1].bounds.fully_contains(&s3_bounds));
    }

    #[test]
    fn optimising_a_csg_should_optimise_its_children() {
        let s1 = Object::sphere().transformed(Transform::identity().translate_x(-1.5));
        let s1_id = s1.id;

        let s2 = Object::sphere().transformed(Transform::identity().translate_x(1.5));
        let s2_id = s2.id;

        let s3 = Object::sphere().transformed(Transform::identity().translate_z(-1.5));
        let s3_id = s3.id;

        let s4 = Object::sphere().transformed(Transform::identity().translate_z(1.5));
        let s4_id = s4.id;

        let csg = Object::csg_difference(Object::group(vec![s1, s2]), Object::group(vec![s3, s4]));
        let optimised = csg.optimised(1);

        let (left, right) = optimised.csg_children();
        assert_eq!(left.children().len(), 2);
        assert_eq!(left.children()[0].children().len(), 1);
        assert_eq!(left.children()[0].children()[0].id, s1_id);

        assert_eq!(left.children()[1].children().len(), 1);
        assert_eq!(left.children()[1].children()[0].id, s2_id);

        assert_eq!(right.children()[0].children().len(), 1);
        assert_eq!(right.children()[0].children()[0].id, s3_id);

        assert_eq!(right.children()[1].children().len(), 1);
        assert_eq!(right.children()[1].children()[0].id, s4_id);
    }

    #[test]
    fn optimising_a_group_with_a_high_threshold_should_have_no_effect() {
        let left_sphere = Object::sphere().transformed(Transform::identity().translate_x(-2.0));
        let left_id = left_sphere.id;

        let right_sphere = Object::sphere().transformed(Transform::identity().translate_x(2.0));
        let right_id = right_sphere.id;

        let middle_sphere = Object::sphere();
        let middle_id = middle_sphere.id;

        let outer = Object::group(vec![left_sphere, right_sphere, middle_sphere]);

        let optimised = outer.optimised(4);
        assert_eq!(optimised.children().len(), 3);
        assert_eq!(optimised.children()[0].id, left_id);
        assert_eq!(optimised.children()[1].id, right_id);
        assert_eq!(optimised.children()[2].id, middle_id);
    }

    #[test]
    fn optimising_a_group_should_create_as_few_subgroups_as_possible() {
        let large = Object::sphere().transformed(Transform::identity().scale_all(2.0));
        let large_id = large.id;

        let right_front =
            Object::sphere().transformed(Transform::identity().translate_x(1.0).translate_z(-0.5));
        let right_front_id = right_front.id;

        let right_back =
            Object::sphere().transformed(Transform::identity().translate_x(1.0).translate_z(0.5));
        let right_back_id = right_back.id;

        let group = Object::group(vec![large, right_front, right_back]);
        let optimised = group.optimised(3);

        assert_eq!(optimised.children().len(), 2);
        assert_eq!(optimised.children()[0].id, large_id);

        assert_eq!(optimised.children()[1].children().len(), 2);
        assert_eq!(optimised.children()[1].children()[0].id, right_front_id);
        assert_eq!(optimised.children()[1].children()[1].id, right_back_id);
    }
}
