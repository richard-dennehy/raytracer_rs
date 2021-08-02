use super::*;
use crate::core::{Normal3D, Point3D, Ray, Vector3D, VectorMaths};
use approx::*;

#[test]
fn a_ray_directly_towards_the_pos_x_face_should_intersect() {
    let cube = Object::cube();
    let ray = Ray::new(Point3D::new(5.0, 0.5, 0.0), Normal3D::NEGATIVE_X);

    let intersections = cube.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 4.0);
    assert_eq!(intersections.get(1).unwrap().t, 6.0);
}

#[test]
fn a_ray_directly_towards_the_neg_x_face_should_intersect() {
    let cube = Object::cube();
    let ray = Ray::new(Point3D::new(-5.0, 0.5, 0.0), Normal3D::POSITIVE_X);

    let intersections = cube.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 4.0);
    assert_eq!(intersections.get(1).unwrap().t, 6.0);
}

#[test]
fn a_ray_directly_towards_the_pos_y_face_should_intersect() {
    let cube = Object::cube();
    let ray = Ray::new(Point3D::new(0.5, 5.0, 0.0), Normal3D::NEGATIVE_Y);

    let intersections = cube.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 4.0);
    assert_eq!(intersections.get(1).unwrap().t, 6.0);
}

#[test]
fn a_ray_directly_towards_the_neg_y_face_should_intersect() {
    let cube = Object::cube();
    let ray = Ray::new(Point3D::new(0.5, -5.0, 0.0), Normal3D::POSITIVE_Y);

    let intersections = cube.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 4.0);
    assert_eq!(intersections.get(1).unwrap().t, 6.0);
}

#[test]
fn a_ray_directly_towards_the_pos_z_face_should_intersect() {
    let cube = Object::cube();
    let ray = Ray::new(Point3D::new(0.5, 0.0, 5.0), Normal3D::NEGATIVE_Z);

    let intersections = cube.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 4.0);
    assert_eq!(intersections.get(1).unwrap().t, 6.0);
}

#[test]
fn a_ray_directly_towards_the_neg_z_face_should_intersect() {
    let cube = Object::cube();
    let ray = Ray::new(Point3D::new(0.5, 0.0, -5.0), Normal3D::POSITIVE_Z);

    let intersections = cube.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 4.0);
    assert_eq!(intersections.get(1).unwrap().t, 6.0);
}

#[test]
fn a_ray_starting_inside_the_cube_should_intersect_in_positive_and_negative_t() {
    let cube = Object::cube();
    let ray = Ray::new(Point3D::new(0.5, 0.0, 0.0), Normal3D::POSITIVE_Z);

    let intersections = cube.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, -1.0);
    assert_eq!(intersections.get(1).unwrap().t, 1.0);
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
            assert_eq!(Object::cube().normal_at(point), normal);
        })
}

#[rustfmt::skip]
#[test]
fn uv_mapping_a_cube_should_project_points_onto_one_of_six_planes() {
    vec![
        ("left",   Point3D::new(-1.0, 0.0, 0.0),   (1.5, 3.5)), // u <- 1..2; v <- 3..4
        ("left",   Point3D::new(-1.0, 0.9, -0.9),  (1.05, 3.95)),
        ("left",   Point3D::new(-1.0, 0.9, 0.9),   (1.95, 3.95)),
        ("left",   Point3D::new(-1.0, -0.9, -0.9), (1.05, 3.05)),
        ("left",   Point3D::new(-1.0, -0.9, 0.9),  (1.95, 3.05)),
        ("front",  Point3D::new(0.0, 0.0, 1.0),    (0.5, 2.5)), // u <- 0..1; v <- 2..3
        ("front",  Point3D::new(-0.9, 0.9, 1.0),   (0.05, 2.95)),
        ("front",  Point3D::new(0.9, 0.9, 1.0),    (0.95, 2.95)),
        ("front",  Point3D::new(-0.9, -0.9, 1.0),  (0.05, 2.05)),
        ("front",  Point3D::new(0.9, -0.9, 1.0),   (0.95, 2.05)),
        ("right",  Point3D::new(1.0, 0.0, 0.0),    (1.5, 1.5)), // u <- 1..2; v <- 1..2
        ("right",  Point3D::new(1.0, 0.9, 0.9),    (1.05, 1.95)),
        ("right",  Point3D::new(1.0, 0.9, -0.9),   (1.95, 1.95)),
        ("right",  Point3D::new(1.0, -0.9, 0.9),   (1.05, 1.05)),
        ("right",  Point3D::new(1.0, -0.9, -0.9),  (1.95, 1.05)),
        ("back",   Point3D::new(0.0, 0.0, -1.0),   (2.5, 2.5)), // u <- 2..3; v <- 2..3
        ("back",   Point3D::new(0.9, 0.9, -1.0),   (2.05, 2.95)),
        ("back",   Point3D::new(-0.9, 0.9, -1.0),  (2.95, 2.95)),
        ("back",   Point3D::new(0.9, -0.9, -1.0),  (2.05, 2.05)),
        ("back",   Point3D::new(-0.9, -0.9, -1.0), (2.95, 2.05)),
        ("top",    Point3D::new(0.0, 1.0, 0.0),    (1.5, 0.5)), // u <- 1..2; v <- 0..1
        ("top",    Point3D::new(-0.9, 1.0, -0.9),  (1.05, 0.95)),
        ("top",    Point3D::new(0.9, 1.0, -0.9),   (1.95, 0.95)),
        ("top",    Point3D::new(-0.9, 1.0, 0.9),   (1.05, 0.05)),
        ("top",    Point3D::new(0.9, 1.0, 0.9),    (1.95, 0.05)),
        ("bottom", Point3D::new(0.0, -1.0, 0.0),   (1.5, 2.5)), // u <- 1..2; v <- 2..3
        ("bottom", Point3D::new(-0.9, -1.0, 0.9),  (1.05, 2.95)),
        ("bottom", Point3D::new(0.9, -1.0, 0.9),   (1.95, 2.95)),
        ("bottom", Point3D::new(-0.9, -1.0, -0.9), (1.05, 2.05)),
        ("bottom", Point3D::new(0.9, -1.0, -0.9),  (1.95, 2.05)),
    ]
        .into_iter()
        .for_each(|(_, point, (u, v))| {
            let (actual_u, actual_v) = Cube.uv_at(point);

            assert_abs_diff_eq!(actual_u, u);
            assert_abs_diff_eq!(actual_v, v);
        })
}
