use super::*;

mod triangle_tests {
    use super::*;
    use crate::core::{Normal3D, Point3D, Ray};
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn the_normal_of_a_triangle_should_be_constant() {
        let triangle = Object::triangle(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );

        let normal = Normal3D::NEGATIVE_Z;

        assert_eq!(triangle.normal_at(Point3D::new(0.0, 0.5, 0.0)), normal);
        assert_eq!(triangle.normal_at(Point3D::new(-0.5, 0.75, 0.0)), normal);
        assert_eq!(triangle.normal_at(Point3D::new(0.5, 0.25, 0.0)), normal);
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
        assert_eq!(intersections.get(0).unwrap().t, 2.0);
    }

    #[test]
    fn uv_mapping_an_xz_triangle_should_project_points_onto_a_plane_described_by_the_edges() {
        let object = Object::triangle(
            Point3D::ORIGIN,
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
        );
        let triangle = object.shape();

        vec![
            (Point3D::ORIGIN, (0.0, 0.0)),
            (Point3D::new(1.0, 0.0, 0.0), (1.0, 0.0)),
            (Point3D::new(0.0, 0.0, 1.0), (0.0, 1.0)),
            (Point3D::new(0.5, 0.0, 0.5), (0.5, 0.5)),
        ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(triangle.uv_at(point), (u, v));
        })
    }

    #[test]
    fn uv_mapping_a_180_degree_rotated_xz_triangle_should_project_points_onto_a_plane_described_by_the_edges(
    ) {
        let object = Object::triangle(
            Point3D::ORIGIN,
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, -1.0),
        );
        let triangle = object.shape();

        vec![
            (Point3D::ORIGIN, (0.0, 0.0)),
            (Point3D::new(-1.0, 0.0, 0.0), (1.0, 0.0)),
            (Point3D::new(0.0, 0.0, -1.0), (0.0, 1.0)),
            (Point3D::new(-0.5, 0.0, -0.5), (0.5, 0.5)),
        ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(triangle.uv_at(point), (u, v));
        })
    }

    #[test]
    fn uv_mapping_a_triangle_on_an_arbitrary_plane_should_project_points_onto_the_plane() {
        // roughly 45 degrees rotated around x then z
        let object = Object::triangle(
            Point3D::ORIGIN,
            Point3D::new(-0.5, 0.5, FRAC_1_SQRT_2),
            Point3D::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0),
        );
        let triangle = object.shape();

        vec![
            (Point3D::ORIGIN, (0.0, 0.0)),
            (Point3D::new(-0.5, 0.5, FRAC_1_SQRT_2), (1.0, 0.0)),
            (Point3D::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0), (0.0, 1.0)),
            (
                Point3D::new(0.103553, 0.603553, 0.353553),
                (0.4999997238088475, 0.4999994476176948),
            ),
        ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(triangle.uv_at(point), (u, v));
        })
    }

    #[test]
    fn uv_mapping_a_non_uniform_right_angle_triangle_should_evenly_project_points_across_the_plane()
    {
        let object = Object::triangle(
            Point3D::ORIGIN,
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, 3.0),
        );
        let triangle = object.shape();

        vec![
            (Point3D::ORIGIN, (0.0, 0.0)),
            (Point3D::new(1.0, 0.0, 0.0), (1.0, 0.0)),
            (Point3D::new(0.0, 0.0, 1.0), (0.0, 1.0 / 3.0)),
            (Point3D::new(0.0, 0.0, 3.0), (0.0, 1.0)),
            (Point3D::new(0.5, 0.0, 1.5), (0.5, 0.5)),
            (Point3D::new(0.5, 0.0, 0.5), (0.5, 1.0 / 6.0)),
        ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(triangle.uv_at(point), (u, v));
        })
    }

    #[test]
    fn uv_mapping_a_non_right_angle_triangle_should_evenly_project_points_across_the_plane() {
        let object = Object::triangle(
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
        );
        let triangle = object.shape();

        vec![
            (Point3D::new(1.0, 0.0, 0.0), (0.0, 0.0)),
            (Point3D::new(0.0, 1.0, 0.0), (1.0, 0.5)),
            (Point3D::new(0.0, 0.0, 1.0), (0.0, 1.0)),
            (Point3D::new(0.5, 0.5, 0.5), (0.5, 0.5)),
            (Point3D::new(0.5, 0.0, 0.5), (0.0, 0.5)),
        ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(triangle.uv_at(point), (u, v));
        })
    }
}

mod smooth_triangles {
    use super::*;
    use crate::core::{Normal3D, Point3D, Ray, Vector, Vector3D};

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
        let intersections = triangle.intersect(&ray);
        let point = ray.position(intersections.get(0).unwrap().t);

        assert_eq!(
            // Point has no effect on normal as u,v is used instead
            triangle.normal_at(point),
            Vector3D::new(-0.554700196225229, 0.8320502943378437, 0.0).normalised()
        );
    }
}
