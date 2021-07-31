use super::*;
use crate::core::{Colour, Normal3D, Point3D, Ray, Transform, Vector, Vector3D};
use approx::*;
use std::f64::consts::SQRT_2;

#[test]
fn should_be_able_to_calculate_the_normal_on_the_x_axis() {
    let sphere = Object::sphere();
    let normal = sphere.normal_at(Point3D::new(1.0, 0.0, 0.0));
    assert_eq!(normal, Normal3D::POSITIVE_X);
}

#[test]
fn should_be_able_to_calculate_the_normal_on_the_y_axis() {
    let sphere = Object::sphere();
    let normal = sphere.normal_at(Point3D::new(0.0, 1.0, 0.0));
    assert_eq!(normal, Normal3D::POSITIVE_Y);
}

#[test]
fn should_be_able_to_calculate_the_normal_on_the_z_axis() {
    let sphere = Object::sphere();
    let normal = sphere.normal_at(Point3D::new(0.0, 0.0, 1.0));
    assert_eq!(normal, Normal3D::POSITIVE_Z);
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
        .normalised()
    );
}

#[test]
fn should_be_able_to_calculate_a_surface_normal_on_a_translated_sphere() {
    use std::f64::consts::FRAC_1_SQRT_2;

    let sphere = Object::sphere().transformed(Transform::identity().translate_y(1.0));

    let normal = sphere.normal_at(Point3D::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    assert_abs_diff_eq!(
        normal,
        Vector3D::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2).normalised()
    );
}

#[test]
fn should_be_able_to_calculate_a_surface_normal_on_a_transformed_sphere() {
    use std::f64::consts::PI;

    let transform = Transform::identity()
        .rotate_z(PI / 5.0)
        .scale_x(1.0)
        .scale_y(0.5)
        .scale_z(1.0);
    let sphere = Object::sphere().transformed(transform);

    let normal = sphere.normal_at(Point3D::new(
        0.0,
        2.0_f64.sqrt() / 2.0,
        -2.0_f64.sqrt() / 2.0,
    ));
    assert_eq!(
        normal,
        Vector3D::new(0.0, 0.9701425001453319, -0.24253562503633294).normalised()
    );
}

#[test]
fn a_ray_passing_through_the_world_origin_should_intersect_a_unit_sphere_at_two_points() {
    let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);
    let sphere = Object::sphere();

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 4.0);
    assert_eq!(intersections.get(1).unwrap().t, 6.0);
}

#[test]
fn a_ray_on_a_tangent_with_a_unit_sphere_should_intersect_twice_at_the_same_point() {
    let ray = Ray::new(Point3D::new(0.0, 1.0, -5.0), Normal3D::POSITIVE_Z);
    let sphere = Object::sphere();

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 5.0);
    assert_eq!(intersections.get(1).unwrap().t, 5.0);
}

#[test]
fn a_ray_passing_over_a_unit_sphere_should_not_intersect() {
    let ray = Ray::new(Point3D::new(0.0, 2.0, -5.0), Normal3D::POSITIVE_Z);
    let sphere = Object::sphere();

    let intersections = sphere.intersect(&ray);
    assert!(intersections.is_empty());
}

#[test]
fn a_ray_originating_inside_a_unit_sphere_should_intersect_in_positive_and_negative_time() {
    let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Normal3D::POSITIVE_Z);
    let sphere = Object::sphere();

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, -1.0);
    assert_eq!(intersections.get(1).unwrap().t, 1.0);
}

#[test]
fn a_ray_originating_outside_a_sphere_and_pointing_away_from_it_should_intersect_twice_in_negative_time(
) {
    let ray = Ray::new(Point3D::new(0.0, 0.0, 5.0), Normal3D::POSITIVE_Z);
    let sphere = Object::sphere();

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, -6.0);
    assert_eq!(intersections.get(1).unwrap().t, -4.0);
}

#[test]
fn a_ray_should_intersect_a_scaled_sphere() {
    let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);
    let sphere = Object::sphere().transformed(Transform::identity().scale_all(2.0));

    let intersections = sphere.intersect(&ray);
    assert_eq!(intersections.len(), 2);

    assert_eq!(intersections.get(0).unwrap().t, 3.0);
    assert_eq!(intersections.get(1).unwrap().t, 7.0);
}

#[test]
fn a_ray_should_not_intersect_a_sphere_translated_away_from_it() {
    let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);
    let translation = Transform::identity().translate_x(5.0);
    let sphere = Object::sphere().transformed(translation);

    let intersections = sphere.intersect(&ray);
    assert!(intersections.is_empty())
}

#[test]
fn lighting_a_point_on_the_left_hemisphere_of_a_default_sphere_with_a_default_stripe_pattern_should_use_the_secondary_colour(
) {
    let sphere = Object::sphere().with_material(Material {
        kind: MaterialKind::Pattern(Pattern::striped(Colour::WHITE, Colour::BLACK)),
        ambient: 1.0,
        diffuse: 0.0,
        specular: 0.0,
        ..Default::default()
    });

    let point = Point3D::new(-0.5, 0.0, 0.0);
    let normal = sphere.normal_at(point);
    let light = Light::point(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0));
    let sample = light.samples().0.next().unwrap();

    assert_eq!(
        sphere.colour_at(
            point,
            Colour::WHITE,
            Normal3D::NEGATIVE_X,
            normal,
            &LightSample::new(*sample, Colour::WHITE),
        ),
        Colour::BLACK
    );
}

#[test]
fn lighting_a_point_on_the_right_hemisphere_of_a_default_sphere_with_a_default_stripe_pattern_should_use_the_primary_colour(
) {
    let sphere = Object::sphere().with_material(Material {
        kind: MaterialKind::Pattern(Pattern::striped(Colour::WHITE, Colour::BLACK)),
        ambient: 1.0,
        diffuse: 0.0,
        specular: 0.0,
        ..Default::default()
    });

    let point = Point3D::new(0.5, 0.0, 0.0);
    let normal = sphere.normal_at(point);
    let light = Light::point(Colour::WHITE, Point3D::new(10.0, 0.0, 0.0));
    let sample = light.samples().0.next().unwrap();

    assert_eq!(
        sphere.colour_at(
            point,
            Colour::WHITE,
            Normal3D::NEGATIVE_X,
            normal,
            &LightSample::new(*sample, Colour::WHITE),
        ),
        Colour::WHITE
    );
}

#[test]
fn uv_mapping_a_sphere_should_project_points_onto_a_plane() {
    vec![
        (Point3D::new(0.0, 0.0, -1.0), (0.0, 0.5)),
        (Point3D::new(1.0, 0.0, 0.0), (0.25, 0.5)),
        (Point3D::new(0.0, 0.0, 1.0), (0.5, 0.5)),
        (Point3D::new(-1.0, 0.0, 0.0), (0.75, 0.5)),
        (Point3D::new(0.0, 1.0, 0.0), (0.5, 1.0)),
        (Point3D::new(0.0, -1.0, 0.0), (0.5, 0.0)),
        (Point3D::new(SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0), (0.25, 0.75)),
    ]
    .into_iter()
    .for_each(|(point, uv)| assert_eq!(Sphere.uv_at(point), uv))
}
