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
        let (first, second) = intersection.unwrap();

        assert_eq!(first.t, 4.0);
        assert_eq!(second.t, 6.0);
    }

    #[test]
    fn a_ray_on_a_tangent_with_a_unit_sphere_should_intersect_twice_at_the_same_point() {
        let ray = Ray::new(Point3D::new(0.0, 1.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let intersection = ray.intersect(&sphere);

        assert!(intersection.is_some());
        let (first, second) = intersection.unwrap();

        assert_eq!(first.t, 5.0);
        assert_eq!(second.t, 5.0);
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
        let (first, second) = intersection.unwrap();

        assert_eq!(first.t, -1.0);
        assert_eq!(second.t, 1.0);
    }

    #[test]
    fn a_ray_originating_outside_a_sphere_and_pointing_away_from_it_should_intersect_twice_in_negative_time(
    ) {
        let ray = Ray::new(Point3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Sphere::unit();

        let intersection = ray.intersect(&sphere);

        assert!(intersection.is_some());
        let (first, second) = intersection.unwrap();

        assert_eq!(first.t, -6.0);
        assert_eq!(second.t, -4.0);
    }

    #[test]
    fn the_hit_of_an_intersection_should_be_the_lowest_positive_t_value() {
        let sphere = Sphere::unit();
        let intersections = Intersections::of(
            Intersection::new(1.0, &sphere),
            Intersection::new(2.0, &sphere),
        );
        let hit = intersections.hit();

        assert!(hit.is_some());
        let hit = hit.unwrap();

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.with, &sphere);
    }

    #[test]
    fn the_hit_of_intersections_should_not_be_the_negative_t_intersection() {
        let sphere = Sphere::unit();
        let intersections = Intersections::of(
            Intersection::new(-1.0, &sphere),
            Intersection::new(1.0, &sphere),
        );
        let hit = intersections.hit();

        assert!(hit.is_some());
        let hit = hit.unwrap();

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.with, &sphere);
    }

    #[test]
    fn the_hit_of_all_negative_intersections_should_be_none() {
        let sphere = Sphere::unit();
        let intersections = Intersections::of(
            Intersection::new(-2.0, &sphere),
            Intersection::new(-1.0, &sphere),
        );
        let hit = intersections.hit();

        assert!(hit.is_none());
    }

    #[test]
    fn the_hit_of_multiple_intersections_should_be_the_lowest_positive_t_value() {
        let sphere = Sphere::unit();
        let intersections = Intersections::of(
            Intersection::new(5.0, &sphere),
            Intersection::new(7.0, &sphere),
        )
        .push(
            Intersection::new(-3.0, &sphere),
            Intersection::new(2.0, &sphere),
        );
        let hit = intersections.hit();

        assert!(hit.is_some());
        let hit = hit.unwrap();

        assert_eq!(hit.t, 2.0);
        assert_eq!(hit.with, &sphere);
    }

    #[test]
    fn a_ray_can_be_translated() {
        let matrix = Matrix4D::translation(3.0, 4.0, 5.0);
        let ray = Ray::new(Point3D::new(1.0, 2.0, 3.0), Vector3D::new(0.0, 1.0, 0.0));

        let transformed = ray.transformed(&matrix);
        assert_eq!(transformed.origin, Point3D::new(4.0, 6.0, 8.0));
        assert_eq!(transformed.direction, Vector3D::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn a_ray_can_be_scaled() {
        let matrix = Matrix4D::scaling(2.0, 3.0, 4.0);
        let ray = Ray::new(Point3D::new(1.0, 2.0, 3.0), Vector3D::new(0.0, 1.0, 0.0));

        let transformed = ray.transformed(&matrix);
        assert_eq!(transformed.origin, Point3D::new(2.0, 6.0, 12.0));
        assert_eq!(transformed.direction, Vector3D::new(0.0, 3.0, 0.0));
    }

    #[test]
    fn a_ray_should_intersect_a_scaled_sphere() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let scale = Matrix4D::scaling(2.0, 2.0, 2.0);
        let mut sphere = Sphere::unit();
        sphere.transform(scale);

        let intersection = ray.intersect(&sphere);
        assert!(intersection.is_some());
        let (first, second) = intersection.unwrap();

        assert_eq!(first.t, 3.0);
        assert_eq!(second.t, 7.0);
    }

    #[test]
    fn a_ray_should_not_intersect_a_sphere_translated_away_from_it() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let translation = Matrix4D::translation(5.0, 0.0, 0.0);
        let mut sphere = Sphere::unit();
        sphere.transform(translation);

        let intersection = ray.intersect(&sphere);
        assert!(intersection.is_none());
    }
}
