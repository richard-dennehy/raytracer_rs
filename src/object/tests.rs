use super::*;

mod shape_tests {
    use super::*;
    use crate::Pattern;
    use std::f64::consts::PI;

    #[test]
    fn lighting_with_the_eye_in_between_the_light_and_the_surface_should_have_full_intensity() {
        let sphere = Object::sphere();
        let point = Point3D::new(0.0, 0.0, -1.0);

        let eye_vector = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));

        let lit_material = sphere.colour_at(point, &light, eye_vector, false);
        assert_eq!(lit_material, Colour::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_at_a_45_degree_angle_to_the_surface_normal_should_have_no_specular() {
        let sphere = Object::sphere();
        let point = Point3D::new(0.0, 0.0, -1.0);

        let eye_vector = Vector3D::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));

        let lit_material = sphere.colour_at(point, &light, eye_vector, false);
        assert_eq!(lit_material, Colour::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_light_at_a_45_degree_angle_to_the_surface_normal_should_have_no_specular_and_less_diffuse(
    ) {
        let mut sphere = Object::sphere();
        sphere.transform = Matrix4D::translation(0.0, 0.0, 1.0);
        let point = Point3D::new(0.0, 0.0, 0.0);

        let eye_vector = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 10.0, -10.0));

        let lit_material = sphere.colour_at(point, &light, eye_vector, false);
        assert_eq!(
            lit_material,
            Colour::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );
    }

    #[test]
    fn lighting_with_the_light_at_45_deg_and_the_eye_at_neg_45_deg_to_the_surface_normal_should_have_less_diffuse(
    ) {
        let mut sphere = Object::sphere();
        sphere.transform = Matrix4D::translation(0.0, 0.0, 1.0);
        let point = Point3D::new(0.0, 0.0, 0.0);

        let eye_vector = Vector3D::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 10.0, -10.0));

        let lit_material = sphere.colour_at(point, &light, eye_vector, false);
        assert_eq!(
            lit_material,
            Colour::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface_should_only_have_ambient() {
        let sphere = Object::sphere();
        let point = Point3D::new(0.0, 0.0, -1.0);

        let eye_vector = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 0.0, 10.0));

        let lit_material = sphere.colour_at(point, &light, eye_vector, false);
        assert_eq!(lit_material, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_a_point_in_shadow_should_only_have_ambient() {
        let sphere = Object::sphere();
        let point = Point3D::new(0.0, 0.0, -1.0);

        let eye_vector = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));

        let lit_material = sphere.colour_at(point, &light, eye_vector, true);
        assert_eq!(lit_material, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn translating_an_object_should_translate_the_pattern_in_world_space() {
        let sphere = Object::sphere()
            .with_transform(Matrix4D::translation(1.0, 0.0, 0.0))
            .with_material(Material {
                pattern: Pattern::striped(Colour::WHITE, Colour::BLACK),
                ambient: 1.0,
                diffuse: 0.0,
                specular: 0.0,
                ..Default::default()
            });

        assert_eq!(
            sphere.colour_at(
                Point3D::new(0.5, 0.0, 0.0),
                &PointLight::new(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0)),
                Vector3D::new(-1.0, 0.0, 0.0),
                false
            ),
            Colour::BLACK
        );
    }

    #[test]
    fn rotating_an_object_should_rotate_the_pattern_in_world_space() {
        let sphere = Object::sphere()
            .with_transform(Matrix4D::rotation_y(PI))
            .with_material(Material {
                pattern: Pattern::striped(Colour::WHITE, Colour::BLACK),
                ambient: 1.0,
                diffuse: 0.0,
                specular: 0.0,
                ..Default::default()
            });

        assert_eq!(
            sphere.colour_at(
                Point3D::new(-0.5, 0.0, 0.0),
                &PointLight::new(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0)),
                Vector3D::new(-1.0, 0.0, 0.0),
                false
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
        let normal = sphere.normal_at(Point3D::new(1.0, 0.0, 0.0));
        assert_eq!(normal, Vector3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_on_the_y_axis() {
        let sphere = Object::sphere();
        let normal = sphere.normal_at(Point3D::new(0.0, 1.0, 0.0));
        assert_eq!(normal, Vector3D::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_on_the_z_axis() {
        let sphere = Object::sphere();
        let normal = sphere.normal_at(Point3D::new(0.0, 0.0, 1.0));
        assert_eq!(normal, Vector3D::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_at_an_arbitrary_point_on_a_sphere() {
        let sphere = Object::sphere();
        let normal = sphere.normal_at(Point3D::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        assert_eq!(
            normal,
            Vector3D::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            )
        );
    }

    #[test]
    fn should_be_able_to_calculate_a_surface_normal_on_a_translated_sphere() {
        use std::f64::consts::FRAC_1_SQRT_2;

        let mut sphere = Object::sphere();
        sphere.transform = Matrix4D::translation(0.0, 1.0, 0.0);

        let normal = sphere.normal_at(Point3D::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        assert!(approx_eq!(
            Vector3D,
            normal,
            Vector3D::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        ));
    }

    #[test]
    fn should_be_able_to_calculate_a_surface_normal_on_a_transformed_sphere() {
        use std::f64::consts::PI;

        let transform = Matrix4D::scaling(1.0, 0.5, 1.0) * Matrix4D::rotation_z(PI / 5.0);
        let mut sphere = Object::sphere();
        sphere.transform = transform;

        let normal = sphere.normal_at(Point3D::new(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));
        assert_eq!(
            normal,
            Vector3D::new(0.0, 0.9701425001453319, -0.24253562503633294)
        );
    }

    #[test]
    fn a_ray_passing_through_the_world_origin_should_intersect_a_unit_sphere_at_two_points() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_on_a_tangent_with_a_unit_sphere_should_intersect_twice_at_the_same_point() {
        let ray = Ray::new(Point3D::new(0.0, 1.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 5.0);
        assert_eq!(intersections.underlying()[1].t, 5.0);
    }

    #[test]
    fn a_ray_passing_over_a_unit_sphere_should_not_intersect() {
        let ray = Ray::new(Point3D::new(0.0, 2.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert!(intersections.underlying().is_empty());
    }

    #[test]
    fn a_ray_originating_inside_a_unit_sphere_should_intersect_in_positive_and_negative_time() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, -1.0);
        assert_eq!(intersections.underlying()[1].t, 1.0);
    }

    #[test]
    fn a_ray_originating_outside_a_sphere_and_pointing_away_from_it_should_intersect_twice_in_negative_time(
    ) {
        let ray = Ray::new(Point3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, -6.0);
        assert_eq!(intersections.underlying()[1].t, -4.0);
    }

    #[test]
    fn a_ray_should_intersect_a_scaled_sphere() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let scale = Matrix4D::scaling(2.0, 2.0, 2.0);
        let mut sphere = Object::sphere();
        sphere.transform = scale;

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 3.0);
        assert_eq!(intersections.underlying()[1].t, 7.0);
    }

    #[test]
    fn a_ray_should_not_intersect_a_sphere_translated_away_from_it() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let translation = Matrix4D::translation(5.0, 0.0, 0.0);
        let mut sphere = Object::sphere();
        sphere.transform = translation;

        let intersections = sphere.intersect(&ray);
        assert!(intersections.underlying().is_empty())
    }

    #[test]
    fn lighting_a_point_on_the_left_hemisphere_of_a_default_sphere_with_a_default_stripe_pattern_should_use_the_secondary_colour(
    ) {
        let sphere = Object::sphere().with_material(Material {
            pattern: Pattern::striped(Colour::WHITE, Colour::BLACK),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        });

        assert_eq!(
            sphere.colour_at(
                Point3D::new(-0.5, 0.0, 0.0),
                &PointLight::new(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0)),
                Vector3D::new(-1.0, 0.0, 0.0),
                false
            ),
            Colour::BLACK
        );
    }

    #[test]
    fn lighting_a_point_on_the_right_hemisphere_of_a_default_sphere_with_a_default_stripe_pattern_should_use_the_primary_colour(
    ) {
        let sphere = Object::sphere().with_material(Material {
            pattern: Pattern::striped(Colour::WHITE, Colour::BLACK),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        });

        assert_eq!(
            sphere.colour_at(
                Point3D::new(0.5, 0.0, 0.0),
                &PointLight::new(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0)),
                Vector3D::new(-1.0, 0.0, 0.0),
                false
            ),
            Colour::WHITE
        );
    }
}

mod plane_tests {
    use super::*;
    use std::f64::consts::PI;

    #[quickcheck]
    fn the_normal_of_an_xz_plane_is_constant_at_all_points(x: f64, z: f64) {
        assert_eq!(
            Object::plane().normal_at(Point3D::new(x, 0.0, z)),
            Vector3D::new(0.0, 1.0, 0.0)
        );
    }

    #[quickcheck]
    fn the_normal_of_an_xy_plane_is_constant_at_all_points(x: f64, y: f64) {
        let plane = Object::plane().with_transform(Matrix4D::rotation_x(PI / 2.0));

        assert!(approx_eq!(
            Vector3D,
            plane.normal_at(Point3D::new(x, y, 0.0)),
            Vector3D::new(0.0, 0.0, 1.0)
        ));
    }

    #[quickcheck]
    fn the_normal_of_a_yz_plane_is_constant_at_all_points(y: f64, z: f64) {
        let plane = Object::plane().with_transform(Matrix4D::rotation_z(PI / 2.0));

        assert!(approx_eq!(
            Vector3D,
            plane.normal_at(Point3D::new(0.0, y, z)),
            Vector3D::new(-1.0, 0.0, 0.0)
        ));
    }

    #[test]
    fn a_plane_is_not_intersected_by_a_parallel_ray() {
        assert!(Object::plane()
            .intersect(&Ray::new(
                Point3D::new(0.0, 1.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0)
            ))
            .is_empty());
    }

    #[test]
    fn a_plane_is_not_intersected_by_a_coplanar_ray() {
        assert!(Object::plane()
            .intersect(&Ray::new(
                Point3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0)
            ))
            .is_empty());
    }

    #[test]
    fn a_plane_is_intersected_by_a_ray_originating_from_above() {
        let plane = Object::plane();
        let intersections = plane.intersect(&Ray::new(
            Point3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, -1.0, 0.0),
        ));

        assert_eq!(intersections.len(), 1);

        assert_eq!(intersections.underlying()[0].t, 1.0);
    }

    #[test]
    fn a_plane_is_intersected_by_a_ray_originating_from_below() {
        let plane = Object::plane();
        let intersections = plane.intersect(&Ray::new(
            Point3D::new(0.0, -1.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
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
        let ray = Ray::new(Point3D::new(5.0, 0.5, 0.0), Vector3D::new(-1.0, 0.0, 0.0));

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_neg_x_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(-5.0, 0.5, 0.0), Vector3D::new(1.0, 0.0, 0.0));

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_pos_y_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, 5.0, 0.0), Vector3D::new(0.0, -1.0, 0.0));

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_neg_y_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, -5.0, 0.0), Vector3D::new(0.0, 1.0, 0.0));

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_pos_z_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, 0.0, 5.0), Vector3D::new(0.0, 0.0, -1.0));

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_directly_towards_the_neg_z_face_should_intersect() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));

        let intersections = cube.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.underlying()[0].t, 4.0);
        assert_eq!(intersections.underlying()[1].t, 6.0);
    }

    #[test]
    fn a_ray_starting_inside_the_cube_should_intersect_in_positive_and_negative_t() {
        let cube = Object::cube();
        let ray = Ray::new(Point3D::new(0.5, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));

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
        let ray = Ray::new(Point3D::new(2.0, 2.0, 0.0), Vector3D::new(-1.0, 0.0, 0.0));

        assert!(cube.intersect(&ray).is_empty());
    }

    #[rustfmt::skip]
    #[test]
    fn the_normal_of_a_cube_point_should_be_based_off_the_largest_component() {
        vec![
            (Point3D::new(1.0, 0.5, -0.8), Vector3D::new(1.0, 0.0, 0.0)),
            (Point3D::new(-1.0, -0.2, 0.9), Vector3D::new(-1.0, 0.0, 0.0)),
            (Point3D::new(-0.4, 1.0, -0.1), Vector3D::new(0.0, 1.0, 0.0)),
            (Point3D::new(0.3, -1.0, -0.7), Vector3D::new(0.0, -1.0, 0.0)),
            (Point3D::new(-0.6, 0.3, 1.0), Vector3D::new(0.0, 0.0, 1.0)),
            (Point3D::new(0.4, 0.4, -1.0), Vector3D::new(0.0, 0.0, -1.0)),
            (Point3D::new(1.0, 1.0, 1.0), Vector3D::new(1.0, 0.0, 0.0)),
            (Point3D::new(-1.0, -1.0, -1.0), Vector3D::new(-1.0, 0.0, 0.0)),
        ]
        .into_iter()
        .for_each(|(point, normal)| {
            assert_eq!(Object::cube().normal_at(point), normal);
        })
    }
}

mod cylinder_tests {
    use super::*;

    #[test]
    fn a_ray_that_misses_an_infinite_cylinder_should_not_intersect() {
        let cylinder = Object::cylinder().build();

        vec![
            Ray::new(Point3D::new(1.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0)),
            Ray::new(Point3D::ORIGIN, Vector3D::new(0.0, 1.0, 0.0)),
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
                Ray::new(Point3D::new(1.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0)),
                5.0,
                5.0,
                "tangent",
            ),
            (
                Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0)),
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
            (Point3D::new(1.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0)),
            (Point3D::new(0.0, 5.0, -1.0), Vector3D::new(0.0, 0.0, -1.0)),
            (Point3D::new(0.0, -2.0, 1.0), Vector3D::new(0.0, 0.0, 1.0)),
            (Point3D::new(-1.0, 1.0, 0.0), Vector3D::new(-1.0, 0.0, 0.0)),
        ]
        .into_iter()
        .for_each(|(point, normal)| {
            assert_eq!(cylinder.normal_at(point), normal);
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

        let ray = Ray::new(Point3D::new(0.0, 1.5, -2.0), Vector3D::new(0.0, 0.0, 1.0));
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
            (Point3D::new(0.0, 1.0, 0.0), Vector3D::new(0.0, -1.0, 0.0)),
            (Point3D::new(0.5, 1.0, 0.0), Vector3D::new(0.0, -1.0, 0.0)),
            (Point3D::new(0.0, 1.0, 0.5), Vector3D::new(0.0, -1.0, 0.0)),
            (Point3D::new(0.0, 2.0, 0.0), Vector3D::new(0.0, 1.0, 0.0)),
            (Point3D::new(0.5, 2.0, 0.0), Vector3D::new(0.0, 1.0, 0.0)),
            (Point3D::new(0.0, 2.0, 0.5), Vector3D::new(0.0, 1.0, 0.0)),
        ]
        .into_iter()
        .for_each(|(point, normal)| {
            assert_eq!(cylinder.normal_at(point), normal);
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
            ("Middle point", Point3D::ORIGIN, Vector3D::new(0.0, 0.0, 0.0)),
            ("Positive y", Point3D::new(1.0, 1.0, 1.0), Vector3D::new(1.0, -SQRT_2, 1.0)),
            ("Negative y", Point3D::new(-1.0, -1.0, 0.0), Vector3D::new(-1.0, 1.0, 0.0)),
        ]
        .into_iter()
        .for_each(|(scenario, point, normal)| {
            // the book examples aren't normalised
            assert_eq!(cone.normal_at(point), normal.normalised(), "{}", scenario);
        })
    }
}
