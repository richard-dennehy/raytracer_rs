use super::*;

mod shape_tests {
    use super::*;
    use crate::core::{Colour, Normal3D, Point3D, Transform, Vector3D, VectorMaths};
    use std::f64::consts::PI;

    #[test]
    fn lighting_with_the_eye_in_between_the_light_and_the_surface_should_have_full_intensity() {
        let sphere = Object::sphere();
        let point = Point3D::new(0.0, 0.0, -1.0);

        let normal = sphere.normal_at(point);
        let eye_vector = Normal3D::NEGATIVE_Z;
        let light = Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));
        let sample = light.samples().0.next().unwrap();

        let lit_material = sphere.colour_at(
            point,
            Colour::WHITE,
            eye_vector,
            normal,
            &LightSample::new(*sample, Colour::WHITE),
        );
        assert_eq!(lit_material, Colour::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_at_a_45_degree_angle_to_the_surface_normal_should_have_no_specular() {
        let sphere = Object::sphere();
        let point = Point3D::new(0.0, 0.0, -1.0);

        let normal = sphere.normal_at(point);
        let eye_vector =
            Vector3D::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0).normalised();
        let light = Light::point(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));
        let sample = light.samples().0.next().unwrap();

        let lit_material = sphere.colour_at(
            point,
            Colour::WHITE,
            eye_vector,
            normal,
            &LightSample::new(*sample, Colour::WHITE),
        );
        assert_eq!(lit_material, Colour::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_light_at_a_45_degree_angle_to_the_surface_normal_should_have_no_specular_and_less_diffuse(
    ) {
        let sphere = Object::sphere().transformed(Transform::identity().translate_z(1.0));
        let point = Point3D::new(0.0, 0.0, 0.0);

        let normal = sphere.normal_at(point);
        let eye_vector = Normal3D::NEGATIVE_Z;
        let light = Light::point(Colour::WHITE, Point3D::new(0.0, 10.0, -10.0));
        let sample = light.samples().0.next().unwrap();

        let lit_material = sphere.colour_at(
            point,
            Colour::WHITE,
            eye_vector,
            normal,
            &LightSample::new(*sample, Colour::WHITE),
        );
        assert_eq!(
            lit_material,
            Colour::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );
    }

    #[test]
    fn lighting_with_the_light_at_45_deg_and_the_eye_at_neg_45_deg_to_the_surface_normal_should_have_less_diffuse(
    ) {
        let sphere = Object::sphere().transformed(Transform::identity().translate_z(1.0));
        let point = Point3D::new(0.0, 0.0, 0.0);

        let normal = sphere.normal_at(point);
        let eye_vector =
            Vector3D::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0).normalised();
        let light = Light::point(Colour::WHITE, Point3D::new(0.0, 10.0, -10.0));
        let sample = light.samples().0.next().unwrap();

        let lit_material = sphere.colour_at(
            point,
            Colour::WHITE,
            eye_vector,
            normal,
            &LightSample::new(*sample, Colour::WHITE),
        );
        assert_eq!(
            lit_material,
            Colour::new(1.6363961030679328, 1.6363961030679328, 1.6363961030679328)
        );
    }

    #[test]
    fn translating_an_object_should_translate_the_pattern_in_world_space() {
        let sphere = Object::sphere()
            .transformed(Transform::identity().translate_x(1.0))
            .with_material(Material {
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
            Colour::BLACK
        );
    }

    #[test]
    fn rotating_an_object_should_rotate_the_pattern_in_world_space() {
        let sphere = Object::sphere()
            .transformed(Transform::identity().rotate_y(PI))
            .with_material(Material {
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
            Colour::WHITE
        );
    }

    #[test]
    fn rotating_a_uv_checkers_pattern_180_degrees_should_invert_it() {
        let checkers_cube = Object::cube().with_material(Material {
            kind: MaterialKind::Uv(
                UvPattern::checkers(
                    Colour::RED,
                    Colour::WHITE,
                    nonzero_ext::nonzero!(2usize),
                    nonzero_ext::nonzero!(2usize),
                )
                .with_transform(Transform::identity().rotate_y(PI)),
            ),
            ..Default::default()
        });

        assert_eq!(
            checkers_cube.raw_colour_at(Point3D::new(-0.5, 0.5, -1.0)),
            Colour::RED
        )
    }

    #[test]
    fn applying_a_planar_uv_to_a_cube_should_wrap_the_plane_around_the_6_faces() {
        let cube = Object::cube().with_material(Material {
            kind: MaterialKind::Uv(UvPattern::checkers(
                Colour::GREEN,
                Colour::WHITE,
                nonzero_ext::nonzero!(2usize),
                nonzero_ext::nonzero!(2usize),
            )),
            ..Default::default()
        });

        vec![
            ("top", Point3D::new(0.1, 1.0, 0.0), Colour::GREEN),
            ("bottom", Point3D::new(0.1, -1.0, 0.0), Colour::GREEN),
            ("left", Point3D::new(-1.0, 0.1, 0.0), Colour::GREEN),
            ("right", Point3D::new(1.0, 0.1, 0.0), Colour::GREEN),
            ("front", Point3D::new(0.1, 0.0, -1.0), Colour::WHITE),
            ("back", Point3D::new(0.1, 0.0, 1.0), Colour::GREEN),
        ]
        .into_iter()
        .for_each(|(scenario, point, expected)| {
            assert_eq!(cube.raw_colour_at(point), expected, "{:?}", scenario)
        })
    }
}

mod group_tests {
    use super::*;
    use crate::core::{Normal3D, Point3D, Ray, Transform, Vector3D, VectorMaths};
    use approx::*;
    use std::f64::consts::PI;

    #[test]
    fn a_ray_should_not_intersect_an_empty_group() {
        let group = Object::group(vec![]);
        let ray = Ray::new(Point3D::ORIGIN, Normal3D::POSITIVE_Z);

        assert!(group.intersect(&ray).is_empty());
    }

    #[test]
    fn a_ray_should_intersect_all_children_in_a_non_empty_group_in_the_path_of_the_ray() {
        let first = Object::sphere();
        let first_id = first.id();

        let second = Object::sphere().transformed(Transform::identity().translate_z(-3.0));
        let second_id = second.id();

        let group = Object::group(vec![
            first,
            second,
            Object::sphere().transformed(Transform::identity().translate_x(5.0)),
        ]);
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = group.intersect(&ray);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections.get(0).unwrap().with.id(), second_id);
        assert_eq!(intersections.get(1).unwrap().with.id(), second_id);
        assert_eq!(intersections.get(2).unwrap().with.id(), first_id);
        assert_eq!(intersections.get(3).unwrap().with.id(), first_id);
    }

    #[test]
    fn a_ray_should_intersect_the_children_of_a_transformed_group() {
        let group = Object::group(vec![
            Object::sphere().transformed(Transform::identity().translate_x(5.0))
        ])
        .transformed(Transform::identity().scale_all(2.0));

        let ray = Ray::new(Point3D::new(10.0, 0.0, -10.0), Normal3D::POSITIVE_Z);
        let intersections = group.intersect(&ray);
        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn group_transforms_should_apply_to_child_normals() {
        let object_transform = Transform::identity().translate_x(5.0);
        let inner_group_transform = Transform::identity().scale_x(1.0).scale_y(2.0).scale_z(3.0);
        let outer_group_transform = Transform::identity().rotate_y(PI / 2.0);

        let group = Object::group(vec![Object::group(vec![
            Object::sphere().transformed(object_transform)
        ])
        .transformed(inner_group_transform)])
        .transformed(outer_group_transform);

        // rust makes getting the reference back to the child sphere awkward, and the book doesn't explain where the point comes from
        // (otherwise it'd be easier to cast a ray to get an Intersection with the sphere)
        let sphere_ref = group
            .children()
            .first()
            .unwrap()
            .children()
            .first()
            .unwrap();

        assert_abs_diff_eq!(
            sphere_ref.normal_at(Point3D::new(1.7321, 1.1547, -5.5774)),
            Vector3D::new(0.28570368184140726, 0.428543151781141, -0.8571605294481017).normalised()
        );
    }
}

mod constructive_solid_geometry {
    use super::*;
    use crate::core::{Normal3D, Point3D, Ray, Transform};

    #[test]
    fn a_ray_that_misses_both_objects_in_a_csg_should_not_intersect() {
        let csg = Object::csg_union(Object::sphere(), Object::cube());
        let ray = Ray::new(Point3D::new(0.0, 2.0, -5.0), Normal3D::POSITIVE_Z);

        assert!(csg.intersect(&ray).is_empty());
    }

    #[test]
    fn a_ray_that_intersects_overlapping_objects_in_a_csg_union_should_intersect_at_the_edge_of_each_object(
    ) {
        let left = Object::sphere();
        let right = Object::sphere().transformed(Transform::identity().translate_z(0.5));

        let left_id = left.id();
        let right_id = right.id();

        let csg = Object::csg_union(left, right);
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = csg.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.get(0).unwrap().t, 4.0);
        assert_eq!(intersections.get(0).unwrap().with.id, left_id);

        assert_eq!(intersections.get(1).unwrap().t, 6.5);
        assert_eq!(intersections.get(1).unwrap().with.id, right_id);
    }

    #[test]
    fn a_ray_that_intersects_overlapping_objects_in_a_csg_intersection_should_intersect_at_the_edges_of_the_overlap(
    ) {
        let left = Object::sphere();
        let right = Object::sphere().transformed(Transform::identity().translate_z(0.5));

        let left_id = left.id();
        let right_id = right.id();

        let csg = Object::csg_intersection(left, right);
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = csg.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.get(0).unwrap().t, 4.5);
        assert_eq!(intersections.get(0).unwrap().with.id, right_id);

        assert_eq!(intersections.get(1).unwrap().t, 6.0);
        assert_eq!(intersections.get(1).unwrap().with.id, left_id);
    }

    #[test]
    fn a_ray_that_intersects_overlapping_objects_in_a_csg_subtraction_should_intersect_exclusively_inside_the_left_object(
    ) {
        let left = Object::sphere();
        let right = Object::sphere().transformed(Transform::identity().translate_z(0.5));

        let left_id = left.id();
        let right_id = right.id();

        let csg = Object::csg_difference(left, right);
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = csg.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.get(0).unwrap().t, 4.0);
        assert_eq!(intersections.get(0).unwrap().with.id, left_id);

        assert_eq!(intersections.get(1).unwrap().t, 4.5);
        assert_eq!(intersections.get(1).unwrap().with.id, right_id);
    }

    // a naive implementation would compare intersection IDs with the IDs of its direct children, but this wouldn't work with Groups and other CSGs
    // this test ensures the implementation isn't that naive
    #[test]
    fn a_csg_comprising_groups_should_correctly_detect_intersections_on_the_children_of_children() {
        let first = Object::sphere().transformed(Transform::identity().translate_z(-3.0));

        let second = Object::sphere().transformed(Transform::identity().translate_z(-0.75));
        let second_id = second.id();

        let third = Object::sphere();
        let third_id = third.id();

        let fourth = Object::sphere().transformed(Transform::identity().translate_z(1.5));

        let csg = Object::csg_intersection(
            Object::group(vec![first, second]),
            Object::csg_difference(third, fourth),
        );
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Normal3D::POSITIVE_Z);

        let intersections = csg.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        assert_eq!(intersections.get(0).unwrap().t, 4.0);
        assert_eq!(intersections.get(0).unwrap().with.id, third_id);

        assert_eq!(intersections.get(1).unwrap().t, 5.25);
        assert_eq!(intersections.get(1).unwrap().with.id, second_id);
    }

    #[test]
    fn transforming_a_csg_should_transform_the_children() {
        let first = Object::sphere().transformed(Transform::identity().translate_x(5.0));
        let first_id = first.id;

        let group = Object::csg_union(first, Object::sphere())
            .transformed(Transform::identity().scale_all(2.0));

        let ray = Ray::new(Point3D::new(10.0, 0.0, -10.0), Normal3D::POSITIVE_Z);
        let intersections = group.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections.get(0).unwrap().with.id, first_id);
    }
}

mod bounding_boxes {
    use super::*;
    use crate::core::{Point3D, Transform};

    #[test]
    fn bounding_box_of_untransformed_primitives() {
        vec![
            (
                "sphere",
                Object::sphere(),
                BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0)),
            ),
            (
                "plane",
                Object::plane(),
                BoundingBox::new(
                    Point3D::new(-f64::MAX, 0.0, -f64::MAX),
                    Point3D::new(f64::MAX, 0.0, f64::MAX),
                ),
            ),
            (
                "cube",
                Object::cube(),
                BoundingBox::new(Point3D::new(-1.0, -1.0, -1.0), Point3D::new(1.0, 1.0, 1.0)),
            ),
            (
                "infinite cylinder",
                Object::cylinder().build(),
                BoundingBox::new(
                    Point3D::new(-1.0, -f64::MAX, -1.0),
                    Point3D::new(1.0, f64::MAX, 1.0),
                ),
            ),
            (
                "truncated cylinder",
                Object::cylinder().min_y(-5.0).max_y(3.0).build(),
                BoundingBox::new(Point3D::new(-1.0, -5.0, -1.0), Point3D::new(1.0, 3.0, 1.0)),
            ),
            (
                "infinite cone",
                Object::cone().build(),
                BoundingBox::new(
                    Point3D::new(-f64::MAX, -f64::MAX, -f64::MAX),
                    Point3D::new(f64::MAX, f64::MAX, f64::MAX),
                ),
            ),
            (
                "truncated cone",
                Object::cone().min_y(-5.0).max_y(3.0).build(),
                BoundingBox::new(Point3D::new(-5.0, -5.0, -5.0), Point3D::new(5.0, 3.0, 5.0)),
            ),
            (
                "triangle",
                Object::triangle(
                    Point3D::new(-3.0, 7.0, 2.0),
                    Point3D::new(6.0, 2.0, -4.0),
                    Point3D::new(2.0, -1.0, -1.0),
                ),
                BoundingBox::new(Point3D::new(-3.0, -1.0, -4.0), Point3D::new(6.0, 7.0, 2.0)),
            ),
        ]
        .into_iter()
        .for_each(|(scenario, object, bounds)| {
            assert_eq!(object.bounds, bounds, "{}", scenario);
        })
    }

    #[test]
    fn the_bounding_box_of_a_transformed_primitive_should_be_transformed() {
        let shape = Object::sphere().transformed(
            Transform::identity()
                .scale_x(0.5)
                .scale_y(2.0)
                .scale_z(4.0)
                .translate_x(1.0)
                .translate_y(-3.0)
                .translate_z(5.0),
        );

        assert_eq!(shape.bounds.min(), Point3D::new(0.5, -5.0, 1.0));
        assert_eq!(shape.bounds.max(), Point3D::new(1.5, -1.0, 9.0));
    }

    #[test]
    fn the_bounding_box_of_a_group_containing_transformed_children_should_contain_the_children_in_world_space(
    ) {
        let group = Object::group(vec![
            Object::sphere().transformed(
                Transform::identity()
                    .scale_all(2.0)
                    .translate_x(2.0)
                    .translate_y(5.0)
                    .translate_z(-3.0),
            ),
            Object::cylinder()
                .max_y(2.0)
                .min_y(-2.0)
                .build()
                .transformed(
                    Transform::identity()
                        .scale_x(0.5)
                        .scale_z(0.5)
                        .translate_x(-4.0)
                        .translate_y(-1.0)
                        .translate_z(4.0),
                ),
        ]);

        assert_eq!(group.bounds.min(), Point3D::new(-4.5, -3.0, -5.0));
        assert_eq!(group.bounds.max(), Point3D::new(4.0, 7.0, 4.5));
    }

    #[test]
    fn the_bounding_box_of_a_csg_should_be_large_enough_to_contain_its_children() {
        let csg = Object::csg_difference(
            Object::sphere(),
            Object::sphere().transformed(
                Transform::identity()
                    .translate_x(2.0)
                    .translate_y(3.0)
                    .translate_z(4.0),
            ),
        );

        assert_eq!(csg.bounds.min(), Point3D::new(-1.0, -1.0, -1.0));
        assert_eq!(csg.bounds.max(), Point3D::new(3.0, 4.0, 5.0));
    }
}

mod optimising_groups {
    use super::*;
    use crate::core::Transform;

    #[test]
    fn optimising_a_group_to_contain_as_few_children_as_possible_should_partition_the_children_based_on_the_split_bounding_box(
    ) {
        let left_sphere = Object::sphere().transformed(Transform::identity().translate_x(-2.0));
        let left_id = left_sphere.id;
        let left_bounds = left_sphere.bounds;

        let right_sphere = Object::sphere().transformed(Transform::identity().translate_x(2.0));
        let right_id = right_sphere.id;
        let right_bounds = right_sphere.bounds;

        let middle_sphere = Object::sphere();
        let middle_id = middle_sphere.id;

        let outer = Object::group(vec![left_sphere, right_sphere, middle_sphere]);
        let outer_bounds = outer.bounds;

        let optimised = outer.optimised(1);

        // ensure original bounds are unchanged
        assert_eq!(optimised.bounds, outer_bounds);

        assert_eq!(optimised.children().len(), 3);
        // ensure middle sphere is a direct child of the new group (as it doesn't entirely fit in either half of the bounds)
        assert_eq!(optimised.children()[0].id(), middle_id);

        // left sphere fits in left half of outer bounds
        assert_eq!(optimised.children()[1].children().len(), 1);
        assert_eq!(optimised.children()[1].children()[0].id(), left_id);
        // ensure the bounds of the sub-group are the same as (i.e. fully contain) the left sphere
        assert_eq!(optimised.children()[1].bounds, left_bounds);

        // right sphere fits in right half of outer bounds
        assert_eq!(optimised.children()[2].children().len(), 1);
        assert_eq!(optimised.children()[2].children()[0].id(), right_id);
        // ensure the bounds of the sub-group are the same as (i.e. fully contain) the right sphere
        assert_eq!(optimised.children()[2].bounds, right_bounds);
    }

    #[test]
    fn optimising_a_group_should_optimise_its_subgroups() {
        let s1 = Object::sphere().transformed(Transform::identity().translate_x(-2.0));
        let s1_id = s1.id;
        let s1_bounds = s1.bounds;

        let s2 =
            Object::sphere().transformed(Transform::identity().translate_x(2.0).translate_y(1.0));
        let s2_id = s2.id;
        let s2_bounds = s2.bounds;

        let s3 =
            Object::sphere().transformed(Transform::identity().translate_x(2.0).translate_y(-1.0));
        let s3_id = s3.id;
        let s3_bounds = s3.bounds;

        let s4 = Object::sphere();
        let s4_id = s4.id;

        let group = Object::group(vec![s4, Object::group(vec![s1, s2, s3])]);
        let optimised = group.optimised(3);

        assert_eq!(optimised.children().len(), 2);
        assert_eq!(optimised.children()[0].id, s4_id);

        let subgroup = &optimised.children()[1];
        assert_eq!(subgroup.children().len(), 2);

        assert_eq!(subgroup.children()[0].children().len(), 1);
        assert_eq!(subgroup.children()[0].children()[0].id, s1_id);
        assert_eq!(subgroup.children()[0].children()[0].bounds, s1_bounds);

        assert_eq!(subgroup.children()[1].children().len(), 2);
        assert_eq!(subgroup.children()[1].children()[0].id, s2_id);
        assert_eq!(subgroup.children()[1].children()[1].id, s3_id);

        assert!(subgroup.children()[1].bounds.fully_contains(&s2_bounds));
        assert!(subgroup.children()[1].bounds.fully_contains(&s3_bounds));
    }

    #[test]
    fn optimising_a_csg_should_optimise_its_children() {
        let s1 = Object::sphere().transformed(Transform::identity().translate_x(-1.5));
        let s1_id = s1.id;

        let s2 = Object::sphere().transformed(Transform::identity().translate_x(1.5));
        let s2_id = s2.id;

        let s3 = Object::sphere().transformed(Transform::identity().translate_z(-1.5));
        let s3_id = s3.id;

        let s4 = Object::sphere().transformed(Transform::identity().translate_z(1.5));
        let s4_id = s4.id;

        let csg = Object::csg_difference(Object::group(vec![s1, s2]), Object::group(vec![s3, s4]));
        let optimised = csg.optimised(1);

        let (left, right) = optimised.csg_children();
        assert_eq!(left.children().len(), 2);
        assert_eq!(left.children()[0].children().len(), 1);
        assert_eq!(left.children()[0].children()[0].id, s1_id);

        assert_eq!(left.children()[1].children().len(), 1);
        assert_eq!(left.children()[1].children()[0].id, s2_id);

        assert_eq!(right.children()[0].children().len(), 1);
        assert_eq!(right.children()[0].children()[0].id, s3_id);

        assert_eq!(right.children()[1].children().len(), 1);
        assert_eq!(right.children()[1].children()[0].id, s4_id);
    }

    #[test]
    fn optimising_a_group_with_a_high_threshold_should_have_no_effect() {
        let left_sphere = Object::sphere().transformed(Transform::identity().translate_x(-2.0));
        let left_id = left_sphere.id;

        let right_sphere = Object::sphere().transformed(Transform::identity().translate_x(2.0));
        let right_id = right_sphere.id;

        let middle_sphere = Object::sphere();
        let middle_id = middle_sphere.id;

        let outer = Object::group(vec![left_sphere, right_sphere, middle_sphere]);

        let optimised = outer.optimised(4);
        assert_eq!(optimised.children().len(), 3);
        assert_eq!(optimised.children()[0].id, left_id);
        assert_eq!(optimised.children()[1].id, right_id);
        assert_eq!(optimised.children()[2].id, middle_id);
    }

    #[test]
    fn optimising_a_group_should_create_as_few_subgroups_as_possible() {
        let large = Object::sphere().transformed(Transform::identity().scale_all(2.0));
        let large_id = large.id;

        let right_front =
            Object::sphere().transformed(Transform::identity().translate_x(1.0).translate_z(-0.5));
        let right_front_id = right_front.id;

        let right_back =
            Object::sphere().transformed(Transform::identity().translate_x(1.0).translate_z(0.5));
        let right_back_id = right_back.id;

        let group = Object::group(vec![large, right_front, right_back]);
        let optimised = group.optimised(3);

        assert_eq!(optimised.children().len(), 2);
        assert_eq!(optimised.children()[0].id, large_id);

        assert_eq!(optimised.children()[1].children().len(), 2);
        assert_eq!(optimised.children()[1].children()[0].id, right_front_id);
        assert_eq!(optimised.children()[1].children()[1].id, right_back_id);
    }

    #[test]
    fn optimising_a_group_containing_only_infinite_shapes_should_have_no_effect() {
        let first = Object::plane();
        let first_id = first.id;

        let second = Object::plane();
        let second_id = second.id;

        let group = Object::group(vec![first, second]);
        let optimised = group.optimised(1);

        assert_eq!(optimised.children().len(), 2);
        assert_eq!(optimised.children()[0].id, first_id);
        assert_eq!(optimised.children()[1].id, second_id);
    }
}
