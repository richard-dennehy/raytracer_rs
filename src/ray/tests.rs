use super::*;

mod ray_unit_tests {
    use super::*;
    use crate::Material;
    use std::f64::consts::SQRT_2;

    #[test]
    fn should_be_able_to_calculate_the_position_of_a_ray_at_a_given_time() {
        let ray = Ray::new(Point3D::new(2.0, 3.0, 4.0), Vector3D::new(1.0, 0.0, 0.0));

        assert_eq!(ray.position(0.0), Point3D::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point3D::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point3D::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point3D::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn the_hit_of_an_intersection_should_be_the_lowest_positive_t_value() {
        let sphere = Object::sphere();
        let intersections = Intersections::of(vec![
            Intersection::new(1.0, &sphere),
            Intersection::new(2.0, &sphere),
        ]);
        let hit = intersections.hit();

        assert!(hit.is_some());
        let hit = hit.unwrap();

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.with, &sphere);
    }

    #[test]
    fn the_hit_of_intersections_should_not_be_the_negative_t_intersection() {
        let sphere = Object::sphere();
        let intersections = Intersections::of(vec![
            Intersection::new(-1.0, &sphere),
            Intersection::new(1.0, &sphere),
        ]);
        let hit = intersections.hit();

        assert!(hit.is_some());
        let hit = hit.unwrap();

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.with, &sphere);
    }

    #[test]
    fn the_hit_of_all_negative_intersections_should_be_none() {
        let sphere = Object::sphere();
        let intersections = Intersections::of(vec![
            Intersection::new(-2.0, &sphere),
            Intersection::new(-1.0, &sphere),
        ]);
        let hit = intersections.hit();

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
    fn should_be_able_to_precompute_hit_data_for_an_outside_hit() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        let intersection = intersections.0[0].clone();

        let data = HitData::from(&ray, intersection, intersections);
        assert_eq!(data.t, 4.0);
        assert_eq!(data.object, &sphere);
        assert_eq!(data.point, Point3D::new(0.0, 0.0, -1.0));
        assert_eq!(data.eye, Vector3D::new(0.0, 0.0, -1.0));
        assert_eq!(data.normal, Vector3D::new(0.0, 0.0, -1.0));
        assert_eq!(data.inside, false);
    }

    #[test]
    fn should_be_able_to_precompute_hit_data_for_an_inside_hit() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        let intersection = intersections.0[1].clone();

        let data = HitData::from(&ray, intersection, intersections);
        assert_eq!(data.t, 1.0);
        assert_eq!(data.object, &sphere);
        assert_eq!(data.point, Point3D::new(0.0, 0.0, 1.0));
        assert_eq!(data.eye, Vector3D::new(0.0, 0.0, -1.0));
        assert_eq!(data.normal, Vector3D::new(0.0, 0.0, -1.0));
        assert!(data.inside);
    }

    #[test]
    fn the_hit_data_should_contain_offset_point_for_shadow_calculations() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere().with_transform(Matrix4D::translation(0.0, 0.0, 1.0));

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        let intersection = intersections.0[0].clone();
        let data = HitData::from(&ray, intersection, intersections);
        assert!(data.over_point.z() < -f64::EPSILON / 2.0);
        assert!(data.point.z() > data.over_point.z());
    }

    #[test]
    fn the_hit_data_should_contain_an_under_offset_point_for_refraction_calculations() {
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = Object::sphere().with_transform(Matrix4D::translation(0.0, 0.0, 1.0));

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);

        let intersection = intersections.0[0].clone();
        let data = HitData::from(&ray, intersection, intersections);
        assert!(data.under_point.z() > -f64::EPSILON / 2.0);
        assert!(data.point.z() < data.under_point.z());
    }

    #[test]
    fn hit_data_should_contain_the_reflection_vector() {
        let ray = Ray::new(
            Point3D::new(0.0, 1.0, -1.0),
            Vector3D::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let sphere = Object::plane();

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 1);

        let intersection = intersections.0[0].clone();
        let data = HitData::from(&ray, intersection, intersections);
        assert_eq!(
            data.reflection,
            Vector3D::new(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)
        );
    }

    #[test]
    fn hit_data_should_calculate_refraction_data() {
        let first = Object::sphere()
            .with_material(Material {
                transparency: 1.0,
                refractive: 1.5,
                ..Default::default()
            })
            .with_transform(Matrix4D::uniform_scaling(2.0));

        let second = Object::sphere()
            .with_material(Material {
                transparency: 1.0,
                refractive: 2.0,
                ..Default::default()
            })
            .with_transform(Matrix4D::translation(0.0, 0.0, -0.25));

        let third = Object::sphere()
            .with_material(Material {
                transparency: 1.0,
                refractive: 2.5,
                ..Default::default()
            })
            .with_transform(Matrix4D::translation(0.0, 0.0, 0.25));

        let ray = Ray::new(Point3D::new(0.0, 0.0, -4.0), Vector3D::new(0.0, 0.0, 1.0));
        let intersections = first
            .intersect(&ray)
            .join(second.intersect(&ray))
            .join(third.intersect(&ray));

        assert_eq!(intersections.len(), 6);

        // enter first sphere
        let hit_data = HitData::from(
            &ray,
            intersections.underlying()[0].clone(),
            intersections.clone(),
        );
        assert_eq!(hit_data.entered_refractive, 1.0);
        assert_eq!(hit_data.exited_refractive, 1.5);

        // enter second sphere (nested in first)
        let hit_data = HitData::from(
            &ray,
            intersections.underlying()[1].clone(),
            intersections.clone(),
        );
        assert_eq!(hit_data.entered_refractive, 1.5);
        assert_eq!(hit_data.exited_refractive, 2.0);

        // enter third sphere (overlapping with second)
        let hit_data = HitData::from(
            &ray,
            intersections.underlying()[2].clone(),
            intersections.clone(),
        );
        assert_eq!(hit_data.entered_refractive, 2.0);
        assert_eq!(hit_data.exited_refractive, 2.5);

        // exit second sphere (still in third sphere)
        let hit_data = HitData::from(
            &ray,
            intersections.underlying()[3].clone(),
            intersections.clone(),
        );
        assert_eq!(hit_data.entered_refractive, 2.5);
        assert_eq!(hit_data.exited_refractive, 2.5);

        // exit third sphere into first
        let hit_data = HitData::from(
            &ray,
            intersections.underlying()[4].clone(),
            intersections.clone(),
        );
        assert_eq!(hit_data.entered_refractive, 2.5);
        assert_eq!(hit_data.exited_refractive, 1.5);

        // exit first sphere into void
        let hit_data = HitData::from(
            &ray,
            intersections.underlying()[5].clone(),
            intersections.clone(),
        );
        assert_eq!(hit_data.entered_refractive, 1.5);
        assert_eq!(hit_data.exited_refractive, 1.0);
    }

    #[test]
    fn the_reflectance_under_total_internal_reflection_should_be_1() {
        let shape = Object::sphere().with_material(Material {
            transparency: 1.0,
            refractive: 1.5,
            reflective: 1.0,
            ..Default::default()
        });

        let ray = Ray::new(
            Point3D::new(0.0, 0.0, SQRT_2 / 2.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        let intersections = shape.intersect(&ray);
        let intersection = intersections.hit();
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        let hit_data = HitData::from(&ray, intersection, intersections);
        assert_eq!(hit_data.reflectance(), 1.0);
    }

    #[test]
    fn the_reflectance_should_be_low_when_the_ray_is_perpendicular() {
        let shape = Object::sphere().with_material(Material {
            transparency: 1.0,
            refractive: 1.5,
            reflective: 1.0,
            ..Default::default()
        });

        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0));

        let intersections = shape.intersect(&ray);
        let intersection = intersections.hit();
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        let hit_data = HitData::from(&ray, intersection, intersections);
        assert_eq!(hit_data.reflectance(), 0.04000000000000001);
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

        let ray = Ray::new(Point3D::new(0.0, 0.99, -2.0), Vector3D::new(0.0, 0.0, 1.0));

        let intersections = shape.intersect(&ray);
        let intersection = intersections.hit();
        assert!(intersection.is_some());
        let intersection = intersection.unwrap();

        let hit_data = HitData::from(&ray, intersection, intersections);
        assert_eq!(hit_data.reflectance(), 0.4888143830387389);
    }
}
