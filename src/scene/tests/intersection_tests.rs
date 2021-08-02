use super::*;
use crate::core::{Normal3D, Point3D, Ray, Transform, Vector, Vector3D};
use approx::*;
use std::f64::consts::SQRT_2;

#[test]
fn the_hit_of_an_intersection_should_be_the_lowest_positive_t_value() {
    let sphere = Object::sphere();
    let intersections = Intersections::of(vec![
        Intersection::new(1.0, &sphere),
        Intersection::new(2.0, &sphere),
    ]);
    let hit = intersections.hit(None);

    assert!(hit.is_some());
    let hit = hit.unwrap();

    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.with.id(), sphere.id());
}

#[test]
fn the_hit_of_intersections_should_not_be_the_negative_t_intersection() {
    let sphere = Object::sphere();
    let intersections = Intersections::of(vec![
        Intersection::new(-1.0, &sphere),
        Intersection::new(1.0, &sphere),
    ]);
    let hit = intersections.hit(None);

    assert!(hit.is_some());
    let hit = hit.unwrap();

    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.with.id(), sphere.id());
}

#[test]
fn the_hit_of_all_negative_intersections_should_be_none() {
    let sphere = Object::sphere();
    let intersections = Intersections::of(vec![
        Intersection::new(-2.0, &sphere),
        Intersection::new(-1.0, &sphere),
    ]);
    let hit = intersections.hit(None);

    assert!(hit.is_none());
}

#[test]
fn the_hit_of_multiple_intersections_should_be_the_lowest_positive_t_value() {
    let sphere = Object::sphere();
    let intersections = Intersections::of(vec![
        Intersection::new(5.0, &sphere),
        Intersection::new(7.0, &sphere),
        Intersection::new(-3.0, &sphere),
        Intersection::new(2.0, &sphere),
    ]);
    let hit = intersections.hit(None);

    assert!(hit.is_some());
    let hit = hit.unwrap();

    assert_eq!(hit.t, 2.0);
    assert_eq!(hit.with.id(), sphere.id());
}

#[test]
fn should_be_able_to_precompute_hit_data_for_an_outside_hit() {
    let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);
    let sphere = Object::sphere();

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    let intersection = intersections.0[0].clone();

    let data = HitData::from(&ray, intersection, intersections);
    assert_eq!(data.object.id(), sphere.id());

    assert_abs_diff_eq!(data.point, Point3D::new(0.0, 0.0, -1.0),);

    assert_eq!(data.eye, Normal3D::NEGATIVE_Z);
    assert_eq!(data.normal, Normal3D::NEGATIVE_Z);
}

#[test]
fn should_be_able_to_precompute_hit_data_for_an_inside_hit() {
    let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Normal3D::POSITIVE_Z);
    let sphere = Object::sphere();

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    let intersection = intersections.0[1].clone();

    let data = HitData::from(&ray, intersection, intersections);
    assert_eq!(data.object.id(), sphere.id());

    assert_abs_diff_eq!(data.point, Point3D::new(0.0, 0.0, 1.0),);
    assert_eq!(data.eye, Normal3D::NEGATIVE_Z);
    assert_eq!(data.normal, Normal3D::NEGATIVE_Z);
}

#[test]
fn hit_data_should_calculate_refraction_data() {
    let first = Object::sphere()
        .with_material(Material {
            transparency: 1.0,
            refractive: 1.5,
            ..Default::default()
        })
        .transformed(Transform::identity().scale_all(2.0));

    let second = Object::sphere()
        .with_material(Material {
            transparency: 1.0,
            refractive: 2.0,
            ..Default::default()
        })
        .transformed(Transform::identity().translate_z(-0.25));

    let third = Object::sphere()
        .with_material(Material {
            transparency: 1.0,
            refractive: 2.5,
            ..Default::default()
        })
        .transformed(Transform::identity().translate_z(0.25));

    let ray = Ray::new(Point3D::new(0.0, 0.0, -4.0), Normal3D::POSITIVE_Z);
    let intersections = first
        .intersect(&ray)
        .join(second.intersect(&ray))
        .join(third.intersect(&ray));

    assert_eq!(intersections.len(), 6);

    // enter first sphere
    let hit_data = HitData::from(
        &ray,
        intersections.get(0).unwrap().clone(),
        intersections.clone(),
    );
    assert_eq!(hit_data.entered_refractive, 1.0);
    assert_eq!(hit_data.exited_refractive, 1.5);

    // enter second sphere (nested in first)
    let hit_data = HitData::from(
        &ray,
        intersections.get(1).unwrap().clone(),
        intersections.clone(),
    );
    assert_eq!(hit_data.entered_refractive, 1.5);
    assert_eq!(hit_data.exited_refractive, 2.0);

    // enter third sphere (overlapping with second)
    let hit_data = HitData::from(
        &ray,
        intersections.get(2).unwrap().clone(),
        intersections.clone(),
    );
    assert_eq!(hit_data.entered_refractive, 2.0);
    assert_eq!(hit_data.exited_refractive, 2.5);

    // exit second sphere (still in third sphere)
    let hit_data = HitData::from(
        &ray,
        intersections.get(3).unwrap().clone(),
        intersections.clone(),
    );
    assert_eq!(hit_data.entered_refractive, 2.5);
    assert_eq!(hit_data.exited_refractive, 2.5);

    // exit third sphere into first
    let hit_data = HitData::from(
        &ray,
        intersections.get(4).unwrap().clone(),
        intersections.clone(),
    );
    assert_eq!(hit_data.entered_refractive, 2.5);
    assert_eq!(hit_data.exited_refractive, 1.5);

    // exit first sphere into void
    let hit_data = HitData::from(
        &ray,
        intersections.get(5).unwrap().clone(),
        intersections.clone(),
    );
    assert_eq!(hit_data.entered_refractive, 1.5);
    assert_eq!(hit_data.exited_refractive, 1.0);
}

#[test]
fn the_hit_data_for_a_smooth_triangle_intersection_should_contain_an_interpolated_normal() {
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
    let intersection = intersections.hit(None);
    assert!(intersection.is_some());
    let hit = HitData::from(&ray, intersection.unwrap(), intersections);
    assert_eq!(
        hit.normal,
        Vector3D::new(-0.554700196225229, 0.8320502943378437, 0.0).normalised()
    );
}

#[test]
fn the_reflectance_under_total_internal_reflection_should_be_1() {
    let shape = Object::sphere().with_material(Material {
        transparency: 1.0,
        refractive: 1.5,
        reflective: 1.0,
        ..Default::default()
    });

    let ray = Ray::new(Point3D::new(0.0, 0.0, SQRT_2 / 2.0), Normal3D::POSITIVE_Y);

    let intersections = shape.intersect(&ray);
    let intersection = intersections.hit(None);
    assert!(intersection.is_some());
    let intersection = intersection.unwrap();

    let hit_data = HitData::from(&ray, intersection, intersections);
    assert_eq!(
        hit_data
            .reflection()
            .reflectance(hit_data.entered_refractive, hit_data.exited_refractive),
        1.0
    );
}

#[test]
fn the_reflectance_should_be_low_when_the_ray_is_perpendicular() {
    let shape = Object::sphere().with_material(Material {
        transparency: 1.0,
        refractive: 1.5,
        reflective: 1.0,
        ..Default::default()
    });

    let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Normal3D::POSITIVE_Y);

    let intersections = shape.intersect(&ray);
    let intersection = intersections.hit(None);
    assert!(intersection.is_some());
    let intersection = intersection.unwrap();

    let hit_data = HitData::from(&ray, intersection, intersections);
    assert_eq!(
        hit_data
            .reflection()
            .reflectance(hit_data.entered_refractive, hit_data.exited_refractive),
        0.04000000000000001
    );
}

#[test]
fn the_reflectance_should_be_significant_when_exiting_a_more_refractive_material_at_a_shallow_angle(
) {
    let shape = Object::sphere().with_material(Material {
        transparency: 1.0,
        refractive: 1.5,
        reflective: 1.0,
        ..Default::default()
    });

    let ray = Ray::new(Point3D::new(0.0, 0.99, -2.0), Normal3D::POSITIVE_Z);

    let intersections = shape.intersect(&ray);
    let intersection = intersections.hit(None);
    assert!(intersection.is_some());
    let intersection = intersection.unwrap();

    let hit_data = HitData::from(&ray, intersection, intersections);
    assert_eq!(
        hit_data
            .reflection()
            .reflectance(hit_data.entered_refractive, hit_data.exited_refractive),
        0.4888143830387389
    );
}

#[test]
fn an_intersection_with_the_same_object_at_a_zero_time_should_not_be_a_hit() {
    let first = Object::plane();
    let second = Object::plane().transformed(Transform::identity().translate_y(0.1));

    let ray = Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Y);

    let intersections = first.intersect(&ray).join(second.intersect(&ray));
    assert_eq!(intersections.len(), 2);

    let hit = intersections.hit(Some(first.id()));
    assert!(hit.is_some());
    assert_eq!(hit.unwrap().with.id(), second.id());
}

#[test]
fn an_intersection_with_the_same_object_at_a_non_zero_t_should_be_a_hit() {
    let object = Object::cube();
    let ray = Ray::new(Point3D::new(0.0, 0.0, -1.0), Normal3D::POSITIVE_Z);

    let intersections = object.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    let hit = intersections.hit(Some(object.id()));
    assert!(hit.is_some());
    assert_eq!(hit.unwrap().t, 2.0);
}
