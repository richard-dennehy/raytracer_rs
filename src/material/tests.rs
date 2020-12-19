use super::*;

mod unit_tests {
    use super::*;

    #[test]
    fn lighting_with_the_eye_in_between_the_light_and_the_surface_should_have_full_intensity() {
        let material = Material::default();
        let eye_position = Point3D::new(0.0, 0.0, 0.0);

        let eye_vector = Vector3D::new(0.0, 0.0, -1.0);
        let surface_normal = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));

        let lit_material = material.with_light(&light, eye_position, eye_vector, surface_normal);
        assert_eq!(lit_material, Colour::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_at_a_45_degree_angle_to_the_surface_normal_should_have_no_specular() {
        let material = Material::default();
        let point = Point3D::new(0.0, 0.0, 0.0);

        let eye_vector = Vector3D::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let surface_normal = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 0.0, -10.0));

        let lit_material = material.with_light(&light, point, eye_vector, surface_normal);
        assert_eq!(lit_material, Colour::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_light_at_a_45_degree_angle_to_the_surface_normal_should_have_no_specular_and_less_diffuse(
    ) {
        let material = Material::default();
        let point = Point3D::new(0.0, 0.0, 0.0);

        let eye_vector = Vector3D::new(0.0, 0.0, -1.0);
        let surface_normal = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 10.0, -10.0));

        let lit_material = material.with_light(&light, point, eye_vector, surface_normal);
        assert_eq!(
            lit_material,
            Colour::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927)
        );
    }

    #[test]
    fn lighting_with_the_light_at_45_deg_and_the_eye_at_neg_45_deg_to_the_surface_normal_should_have_less_diffuse(
    ) {
        let material = Material::default();
        let point = Point3D::new(0.0, 0.0, 0.0);

        let eye_vector = Vector3D::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let surface_normal = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 10.0, -10.0));

        let lit_material = material.with_light(&light, point, eye_vector, surface_normal);
        assert_eq!(
            lit_material,
            Colour::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface_should_only_have_ambient() {
        let material = Material::default();
        let point = Point3D::new(0.0, 0.0, 0.0);

        let eye_vector = Vector3D::new(0.0, 0.0, -1.0);
        let surface_normal = Vector3D::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Colour::WHITE, Point3D::new(0.0, 0.0, 10.0));

        let lit_material = material.with_light(&light, point, eye_vector, surface_normal);
        assert_eq!(lit_material, Colour::new(0.1, 0.1, 0.1));
    }
}
