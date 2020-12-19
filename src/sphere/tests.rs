use super::*;

mod unit_tests {
    use super::*;

    #[test]
    fn should_be_able_to_calculate_the_normal_on_the_x_axis() {
        let sphere = Sphere::unit();
        let normal = sphere.normal_at(Point3D::new(1.0, 0.0, 0.0));
        assert_eq!(normal, Vector3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_on_the_y_axis() {
        let sphere = Sphere::unit();
        let normal = sphere.normal_at(Point3D::new(0.0, 1.0, 0.0));
        assert_eq!(normal, Vector3D::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_on_the_z_axis() {
        let sphere = Sphere::unit();
        let normal = sphere.normal_at(Point3D::new(0.0, 0.0, 1.0));
        assert_eq!(normal, Vector3D::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn should_be_able_to_calculate_the_normal_at_an_arbitrary_point_on_a_sphere() {
        let sphere = Sphere::unit();
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

        let mut sphere = Sphere::unit();
        sphere.transform(Matrix4D::translation(0.0, 1.0, 0.0));

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
        let mut sphere = Sphere::unit();
        sphere.transform(transform);

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
}
