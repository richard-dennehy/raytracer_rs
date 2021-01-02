use super::*;

mod shape_tests {
    use super::*;

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
}

mod sphere_tests {
    use super::*;

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
}
