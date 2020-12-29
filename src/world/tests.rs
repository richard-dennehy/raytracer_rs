use super::*;

mod unit_tests {
    use super::*;
    use crate::Vector3D;

    #[test]
    fn intersecting_a_ray_with_the_default_world_should_produce_a_sorted_list_of_intersections() {
        let world = World::default();
        let intersections = world.intersect(&Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        ));

        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections.get(0).unwrap().t, 4.0);
        assert_eq!(intersections.get(1).unwrap().t, 4.5);
        assert_eq!(intersections.get(2).unwrap().t, 5.5);
        assert_eq!(intersections.get(3).unwrap().t, 6.0);
    }

    #[test]
    fn should_correctly_shade_an_external_hit() {
        let world = World::default();
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = world
            .objects
            .first()
            .expect("Default world should have objects");
        let intersection = ray.intersect(sphere);
        assert!(intersection.is_some());
        let (intersection, _) = intersection.unwrap();
        let hit_data = ray.hit_data(intersection);
        let colour = world.shade_hit(&hit_data);

        assert_eq!(
            colour,
            Colour::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }

    #[test]
    fn should_correctly_shade_an_internal_hit() {
        let mut world = World::default();
        world.lights = vec![PointLight::new(Colour::WHITE, Point3D::new(0.0, 0.25, 0.0))];

        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let sphere = world
            .objects
            .get(1)
            .expect("Default world should have objects");

        let intersection = ray.intersect(sphere);
        assert!(intersection.is_some());

        let (_, intersection) = intersection.unwrap();
        assert!(intersection.t >= 0.0);

        let hit_data = ray.hit_data(intersection);
        let colour = world.shade_hit(&hit_data);

        assert_eq!(colour, Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_colour_should_be_black_when_a_ray_hits_nothing() {
        let world = World::default();
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 1.0, 0.0));

        assert_eq!(world.colour_at(ray), Colour::BLACK);
    }

    #[test]
    fn the_colour_should_be_the_shaded_surface_when_the_ray_hits_an_object() {
        let world = World::default();
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Vector3D::new(0.0, 0.0, 1.0));

        assert_eq!(
            world.colour_at(ray),
            Colour::new(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }

    #[test]
    fn the_colour_should_not_include_any_objects_behind_the_ray() {
        let mut world = World::default();
        world
            .objects
            .iter_mut()
            .for_each(|obj| obj.material.ambient = 1.0);

        let ray = Ray::new(Point3D::new(0.0, 0.0, 0.75), Vector3D::new(0.0, 0.0, -1.0));

        assert_eq!(world.colour_at(ray), Colour::WHITE);
    }
}
