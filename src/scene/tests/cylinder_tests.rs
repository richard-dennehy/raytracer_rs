use super::*;
use crate::core::{Normal3D, Point3D, Ray, Vector3D, VectorMaths};
use std::f64::consts::FRAC_1_SQRT_2;

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
        assert_eq!(intersections.get(0).unwrap().t, t0, "{}", scenario);
        assert_eq!(intersections.get(1).unwrap().t, t1, "{}", scenario);
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

    let ray = Ray::new(Point3D::new(0.0, 1.5, -2.0), Normal3D::POSITIVE_Z);
    let intersections = cylinder.intersect(&ray);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections.get(0).unwrap().t, 1.0);
    assert_eq!(intersections.get(1).unwrap().t, 3.0);
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
        assert_eq!(cylinder.normal_at(point), normal);
    })
}

#[rustfmt::skip]
#[test]
fn uv_mapping_a_unit_cylinder_should_project_points_on_the_sides_onto_a_plane() {
    let cylinder = Object::cylinder().min_y(0.0).max_y(1.0).build();

    vec![
        (Point3D::new(0.0, 0.0, -1.0),                      (0.0, 0.0)),
        (Point3D::new(0.0, 0.5, -1.0),                      (0.0, 0.5)),
        (Point3D::new(0.0, 1.0, -1.0),                      (0.0, 0.0)),
        (Point3D::new(FRAC_1_SQRT_2, 0.5, -FRAC_1_SQRT_2),  (0.125, 0.5)),
        (Point3D::new(1.0, 0.5, 0.0),                       (0.25, 0.5)),
        (Point3D::new(FRAC_1_SQRT_2, 0.5, FRAC_1_SQRT_2),   (0.375, 0.5)),
        (Point3D::new(0.0, -0.25, 1.0),                     (0.5, 0.75)),
        (Point3D::new(-FRAC_1_SQRT_2, 0.5, FRAC_1_SQRT_2),  (0.625, 0.5)),
        (Point3D::new(-1.0, 1.25, 0.0),                     (0.75, 0.25)),
        (Point3D::new(-FRAC_1_SQRT_2, 0.5, -FRAC_1_SQRT_2), (0.875, 0.5)),
    ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(cylinder.shape().uv_at(point), (u, v));
        })
}

#[rustfmt::skip]
#[test]
fn uv_mapping_the_caps_of_a_capped_cylinder_should_project_points_onto_a_circle_on_a_plane() {
    let cylinder = Object::cylinder().min_y(0.0).max_y(1.0).capped().build();

    vec![
        (Point3D::new(0.0, 1.0, 0.0), (1.5, 0.5)),
        (Point3D::new(-1.0, 1.0, 0.0), (1.0, 0.5)),
        (Point3D::new(1.0, 1.0, 0.0), (2.0, 0.5)),
        (Point3D::new(0.0, 1.0, -1.0), (1.5, 1.0)),
        (Point3D::new(0.0, 1.0, 1.0), (1.5, 0.0)),
        (Point3D::new(FRAC_1_SQRT_2, 1.0, FRAC_1_SQRT_2), (1.8535533905932737, 0.1464466094067262)),
        (Point3D::new(0.0, 0.0, 0.0), (2.5, 0.5)),
        (Point3D::new(-1.0, 0.0, 0.0), (2.0, 0.5)),
        (Point3D::new(1.0, 0.0, 0.0), (3.0, 0.5)),
        (Point3D::new(0.0, 0.0, -1.0), (2.5, 0.0)),
        (Point3D::new(0.0, 0.0, 1.0), (2.5, 1.0)),
        (Point3D::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2), (2.853553390593274, 0.8535533905932737)),
    ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(cylinder.shape().uv_at(point), (u, v));
        })
}
