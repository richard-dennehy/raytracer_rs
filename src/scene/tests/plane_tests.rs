use super::*;
use crate::core::{Normal3D, Point3D, Ray, Transform};
use approx::*;
use quickcheck_macros::quickcheck;
use std::f64::consts::PI;

#[quickcheck]
fn the_normal_of_an_xz_plane_is_constant_at_all_points(x: f64, z: f64) {
    assert_eq!(
        Object::plane().normal_at(Point3D::new(x, 0.0, z)),
        Normal3D::POSITIVE_Y
    );
}

#[quickcheck]
fn the_normal_of_an_xy_plane_is_constant_at_all_points(x: f64, y: f64) {
    let plane = Object::plane().transformed(Transform::identity().rotate_x(PI / 2.0));

    assert_abs_diff_eq!(
        plane.normal_at(Point3D::new(x, y, 0.0)),
        Normal3D::POSITIVE_Z
    );
}

#[quickcheck]
fn the_normal_of_a_yz_plane_is_constant_at_all_points(y: f64, z: f64) {
    let plane = Object::plane().transformed(Transform::identity().rotate_z(PI / 2.0));

    assert_abs_diff_eq!(
        plane.normal_at(Point3D::new(0.0, y, z)),
        Normal3D::NEGATIVE_X
    );
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

    assert_eq!(intersections.get(0).unwrap().t, 1.0);
}

#[test]
fn a_plane_is_intersected_by_a_ray_originating_from_below() {
    let plane = Object::plane();
    let intersections = plane.intersect(&Ray::new(
        Point3D::new(0.0, -1.0, 0.0),
        Normal3D::POSITIVE_Y,
    ));

    assert_eq!(intersections.len(), 1);

    assert_eq!(intersections.get(0).unwrap().t, 1.0);
}

#[test]
fn uv_mapping_a_plane_should_return_the_fraction_of_the_x_and_z() {
    vec![
        (Point3D::new(0.25, 0.0, 0.5), (0.25, 0.5)),
        (Point3D::new(0.25, 0.0, -0.25), (0.25, 0.75)),
        (Point3D::new(0.25, 0.5, -0.25), (0.25, 0.75)),
        (Point3D::new(1.25, 0.0, 0.5), (0.25, 0.5)),
        (Point3D::new(0.25, 0.0, -1.75), (0.25, 0.25)),
        (Point3D::new(1.0, 0.0, -1.0), (0.0, 0.0)),
        (Point3D::ORIGIN, (0.0, 0.0)),
    ]
    .into_iter()
    .for_each(|(point, (u, v))| {
        assert_eq!(Plane.uv_at(point), (u, v));
    })
}
