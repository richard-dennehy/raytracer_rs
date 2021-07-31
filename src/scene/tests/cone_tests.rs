use super::*;
use crate::core::{Normal3D, Point3D, Ray, Vector, Vector3D};
use approx::*;
use std::f64::consts::{FRAC_1_SQRT_2, SQRT_2};

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
        assert_eq!(intersections.get(0).unwrap().t, first, "{}", scenario);
        assert_eq!(intersections.get(1).unwrap().t, second, "{}", scenario);
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
    assert_eq!(intersections.get(0).unwrap().t, 0.3535533905932738);
}

#[test]
fn a_ray_should_be_able_to_intersect_the_caps_of_a_capped_cone() {
    let cone = Object::cone().min_y(-0.5).max_y(0.5).capped().build();

    vec![
        (
            "Misses cone",
            Point3D::new(0.51, -5.0, 0.0),
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
fn the_size_of_the_top_cap_of_a_capped_cone_should_scale_with_the_max_y_value() {
    let cone = Object::cone().min_y(0.0).max_y(2.0).capped().build();
    let ray = Ray::new(Point3D::new(0.0, 3.0, 1.5), Normal3D::NEGATIVE_Y);

    let intersections = cone.intersect(&ray);
    assert_eq!(intersections.len(), 2);
}

#[rustfmt::skip]
#[test]
fn should_be_able_to_calculate_the_normal_of_any_point_on_a_double_napped_cone() {
    let cone = Object::cone().build();

    vec![
        ("Middle point", Point3D::ORIGIN, Vector3D::new(0.0, 0.0, 0.0).normalised()),
        ("Positive y", Point3D::new(1.0, 1.0, 1.0), Vector3D::new(1.0, -SQRT_2, 1.0).normalised()),
        ("Negative y", Point3D::new(-1.0, -1.0, 0.0), Vector3D::new(-1.0, 1.0, 0.0).normalised()),
    ]
        .into_iter()
        .for_each(|(_, point, normal)| {
            assert_abs_diff_eq!(cone.normal_at(point), normal);
        })
}

#[rustfmt::skip]
#[test]
fn uv_mapping_a_unit_cone_should_project_points_on_the_sides_onto_a_plane() {
    let cone = Object::cone().min_y(-2.0).max_y(0.0).build();
    let _45_deg = FRAC_1_SQRT_2 / 2.0;

    vec![
        (Point3D::new(0.0, 0.0, 0.0),            (0.5, 0.0)),
        (Point3D::new(0.0, 0.5, -0.5),           (0.0, 0.5)),
        (Point3D::new(0.0, -1.0, -1.0),          (0.0, 0.0)),
        (Point3D::new(_45_deg, -0.5, -_45_deg),  (0.125, 0.5)),
        (Point3D::new(0.5, -0.5, 0.0),           (0.25, 0.5)),
        (Point3D::new(_45_deg, -0.5, _45_deg),   (0.375, 0.5)),
        (Point3D::new(0.0, -0.25, 0.25),         (0.5, 0.75)),
        (Point3D::new(-_45_deg, -0.5, _45_deg),  (0.625, 0.5)),
        (Point3D::new(-1.25, -1.25, 0.0),        (0.75, 0.75)),
        (Point3D::new(-_45_deg, -0.5, -_45_deg), (0.875, 0.5)),
    ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(cone.shape().uv_at(point), (u, v));
        })
}

#[rustfmt::skip]
#[test]
fn uv_mapping_the_caps_of_a_capped_cone_should_project_points_onto_a_circle_on_a_plane() {
    let cone = Object::cone().min_y(-1.0).max_y(1.0).capped().build();

    vec![
        (Point3D::new(0.0, 1.0, 0.0), (1.5, 0.5)),
        (Point3D::new(-1.0, 1.0, 0.0), (1.0, 0.5)),
        (Point3D::new(1.0, 1.0, 0.0), (2.0, 0.5)),
        (Point3D::new(0.0, 1.0, -1.0), (1.5, 1.0)),
        (Point3D::new(0.0, 1.0, 1.0), (1.5, 0.0)),
        (Point3D::new(FRAC_1_SQRT_2, 1.0, FRAC_1_SQRT_2), (1.8535533905932737, 0.1464466094067262)),
        (Point3D::new(0.0, -1.0, 0.0), (2.5, 0.5)),
        (Point3D::new(-1.0, -1.0, 0.0), (2.0, 0.5)),
        (Point3D::new(1.0, -1.0, 0.0), (3.0, 0.5)),
        (Point3D::new(0.0, -1.0, -1.0), (2.5, 0.0)),
        (Point3D::new(0.0, -1.0, 1.0), (2.5, 1.0)),
        (Point3D::new(FRAC_1_SQRT_2, -1.0, FRAC_1_SQRT_2), (2.853553390593274, 0.8535533905932737)),
    ]
        .into_iter()
        .for_each(|(point, (u, v))| {
            assert_eq!(cone.shape().uv_at(point), (u, v));
        })
}
