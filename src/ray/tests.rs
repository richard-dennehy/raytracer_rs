use super::*;

mod ray_unit_tests {
    use super::*;

    #[test]
    fn should_be_able_to_calculate_the_position_of_a_ray_at_a_given_time() {
        let ray = Ray::new(Point3D::new(2.0, 3.0, 4.0), Vector3D::new(1.0, 0.0, 0.0));

        assert_eq!(ray.position(0.0), Point3D::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point3D::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point3D::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point3D::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn a_ray_passing_through_the_world_origin_should_intersect_a_unit_sphere_at_two_points() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let intersection = ray.intersect(&sphere);
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        assert_eq!(intersection.first, 4.0);
        assert_eq!(intersection.second, 6.0);
    }

    #[test]
    fn a_ray_on_a_tangent_with_a_unit_sphere_should_intersect_twice_at_the_same_point() {
        let ray = Ray::new(Point3D::new(0.0, 1.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let intersection = ray.intersect(&sphere);

        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        assert_eq!(intersection.first, 5.0);
        assert_eq!(intersection.second, 5.0);
    }

    #[test]
    fn a_ray_passing_over_a_unit_sphere_should_not_intersect() {
        let ray = Ray::new(Point3D::new(0.0, 2.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let intersection = ray.intersect(&sphere);

        assert!(intersection.is_none());
    }

    #[test]
    fn a_ray_originating_inside_a_unit_sphere_should_intersect_in_positive_and_negative_time() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let intersection = ray.intersect(&sphere);

        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        assert_eq!(intersection.first, -1.0);
        assert_eq!(intersection.second, 1.0);
    }

    #[test]
    fn a_ray_originating_outside_a_sphere_and_pointing_away_from_it_should_intersect_twice_in_negative_time(
    ) {
        let ray = Ray::new(Point3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let intersection = ray.intersect(&sphere);

        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        assert_eq!(intersection.first, -6.0);
        assert_eq!(intersection.second, -4.0);
    }
}
