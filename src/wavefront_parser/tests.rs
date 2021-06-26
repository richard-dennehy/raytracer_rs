use super::*;

mod mtl_parser_tests {
    use super::*;
    use crate::util::F64Ext;

    #[test]
    fn parser_should_ignore_unrecognised_lines() {
        let invalid_file = "this isn't a valid mtl
file in any way,
but this parser is very lazy
and doesn't care about that
¯\\_(ツ)_/¯";

        let out = parse_mtl(invalid_file);
        assert_eq!(out, HashMap::new());
    }

    mod single_material {
        use super::*;

        #[test]
        #[should_panic]
        fn each_material_must_be_named() {
            let input = "
# oh no it's commented out
# newmtl awful_green
Ns 225.000000
Ka 1.000000 1.000000 1.000000
Kd 0.425245 0.800000 0.011982
Ks 0.500000 0.500000 0.500000
Ke 0.000000 0.000000 0.000000
Ni 1.450000
d 1.000000
illum 2";

            parse_mtl(input);
        }

        #[test]
        fn a_kd_rgb_statement_should_define_the_material_colour() {
            let input = "
newmtl awful_green
Kd 0.425245 0.800000 0.011982";

            let materials = parse_mtl(input);
            let green = materials.get("awful_green");
            assert!(green.is_some());
            let green = green.unwrap();
            assert_eq!(
                green.kind,
                MaterialKind::Solid(Colour::new(0.425245, 0.8, 0.011982))
            );
        }

        #[test]
        fn a_kd_statement_with_a_single_value_should_define_the_material_greyscale_colour() {
            let input = "
newmtl overcast_grey
Kd 0.7";

            let materials = parse_mtl(input);
            let grey = materials.get("overcast_grey");
            assert!(grey.is_some());
            let grey = grey.unwrap();
            assert_eq!(grey.kind, MaterialKind::Solid(Colour::greyscale(0.7)));
        }

        #[test]
        fn an_ns_statement_should_define_the_materials_shininess() {
            let input = "
newmtl blinding
Ns 1000";

            let materials = parse_mtl(input);
            let material = materials.get("blinding");
            assert!(material.is_some());
            let material = material.unwrap();
            assert_eq!(material.shininess, 1000.0);
        }

        #[test]
        fn a_ka_rgb_statement_should_define_the_ambient_value() {
            vec![
                ("Ka 1 1 1", 0.1),
                ("Ka 1", 0.1),
                ("Ka 0.0000 1.0000 0.0000", 0.1),
                ("Ka 0.0000 0.0000 1.0000", 0.1),
                ("Ka 0.0000 0.0000 0.0000", 0.0),
                ("Ka 0.5", 0.05),
            ]
            .into_iter()
            .for_each(|(ka, ambient)| {
                let input = format!(
                    "
newmtl ambient
{}",
                    ka
                );

                let materials = parse_mtl(&input);
                let material = materials.get("ambient");
                assert!(material.is_some());
                let material = material.unwrap();
                assert_eq!(material.ambient, ambient);
            })
        }

        #[test]
        fn a_ks_rgb_statement_should_define_the_specular_value() {
            vec![
                ("Ks 1 1 1", 1.0),
                ("Ks 1", 1.0),
                ("Ks 0.0000 1.0000 0.0000", 1.0),
                ("Ks 0.0000 0.0000 1.0000", 1.0),
                ("Ks 0.0000 0.0000 0.0000", 0.0),
                ("Ks 0.5", 0.5),
            ]
            .into_iter()
            .for_each(|(ks, specular)| {
                let input = format!(
                    "
newmtl specular
{}",
                    ks
                );

                let materials = parse_mtl(&input);
                let material = materials.get("specular");
                assert!(material.is_some());
                let material = material.unwrap();
                assert_eq!(material.specular, specular);
            })
        }

        #[test]
        fn an_ni_statement_should_define_the_refractive_index() {
            let input = "
newmtl refractive
Ni 1.45";

            let materials = parse_mtl(input);
            let material = materials.get("refractive");
            assert!(material.is_some());
            let material = material.unwrap();
            assert_eq!(material.refractive, 1.45);
        }

        #[test]
        fn a_mtl_with_a_d_of_1_should_be_fully_opaque() {
            let input = "
newmtl opaque
d 1.000";

            let materials = parse_mtl(input);
            let material = materials.get("opaque");
            assert!(material.is_some());
            let material = material.unwrap();
            assert_eq!(material.transparency, 0.0);
        }

        #[test]
        fn a_mtl_with_a_d_of_0_should_be_fully_transparent() {
            let input = "
newmtl transparent
d 0.000";

            let materials = parse_mtl(input);
            let material = materials.get("transparent");
            assert!(material.is_some());
            let material = material.unwrap();
            assert_eq!(material.transparency, 1.0);
        }

        #[test]
        fn an_illum_statement_should_alter_the_light_interactions() {
            vec![
                // very simplified:
                // illum 0 - solid colour (full ambience, no diffuse or specular)
                // illum 1 - diffuse + ambient, no specular
                // illum 2 - phong + lambertian (normal operation)
                // illum 3 & 8 - reflective
                // illum 4 - 7 - reflective & transparent
                // illum 9 - transparent
                // illum 10 - "Casts shadows onto invisible surfaces" ??? - definitely not supported
                (1.0, 0.0, 0.0, 0.0, 0.0),
                (0.1, 0.9, 0.0, 0.0, 0.0),
                (0.1, 0.9, 0.9, 0.0, 0.0),
                (0.1, 0.9, 0.9, 0.0, 1.0),
                (0.1, 0.9, 0.9, 1.0, 1.0),
                (0.1, 0.9, 0.9, 1.0, 1.0),
                (0.1, 0.9, 0.9, 1.0, 1.0),
                (0.1, 0.9, 0.9, 1.0, 1.0),
                (0.1, 0.9, 0.9, 0.0, 1.0),
                (0.1, 0.9, 0.9, 1.0, 0.0),
            ]
            .into_iter()
            .enumerate()
            .for_each(
                |(idx, (ambient, diffuse, specular, transparency, reflectivity))| {
                    let input = format!(
                        "
newmtl illum
illum {}",
                        idx
                    );

                    let materials = parse_mtl(&input);
                    let material = materials.get("illum");
                    assert!(material.is_some());
                    let material = material.unwrap();
                    assert_eq!(material.ambient, ambient, "({})", idx);
                    assert_eq!(material.diffuse, diffuse, "({})", idx);
                    assert_eq!(material.specular, specular, "({})", idx);
                    assert_eq!(material.transparency, transparency, "({})", idx);
                    assert_eq!(material.reflective, reflectivity, "({})", idx);
                },
            )
        }

        #[test]
        fn an_mtl_material_should_be_converted_into_a_similar_raytracer_material() {
            let input = "
newmtl awful_green
Ns 225.000000
Ka 1.000000 1.000000 1.000000
Kd 0.425245 0.800000 0.011982
Ks 0.500000 0.500000 0.500000
Ke 0.000000 0.000000 0.000000
Ni 1.450000
d 1.000000
illum 2";

            let materials = parse_mtl(input);
            let material = materials.get("awful_green");
            assert!(material.is_some());
            let material = material.unwrap();
            assert_eq!(material.shininess, 225.0);
            assert_eq!(material.ambient, 0.1);
            assert_eq!(
                material.kind,
                MaterialKind::Solid(Colour::new(0.425245, 0.8, 0.011982))
            );
            assert_eq!(material.specular, 0.5);
            assert_eq!(material.refractive, 1.45);
            assert_eq!(material.transparency, 0.0);
            assert_eq!(material.reflective, 0.0);
        }
    }

    #[test]
    fn should_be_able_to_parse_mtl_file_containing_multiple_materials() {
        let input = "
# from https://www.fileformat.info/format/material/

newmtl neon_green
Kd 0.0000 1.0000 0.0000
illum 0

newmtl flat_green
Ka 0.0000 1.0000 0.0000
Kd 0.0000 1.0000 0.0000
illum 1

newmtl diss_green
Ka 0.0000 1.0000 0.0000
Kd 0.0000 1.0000 0.0000
d 0.8000
illum 1

newmtl shiny_green
Ka 0.0000 1.0000 0.0000
Kd 0.0000 1.0000 0.0000
Ks 1.0000 1.0000 1.0000
Ns 200.0000
# note: this uses illum model 1 in the original, which is likely a mistake, as 1 doesn't have specular highlights enabled
illum 2

newmtl green_mirror
Ka 0.0000 1.0000 0.0000
Kd 0.0000 1.0000 0.0000
Ks 0.0000 1.0000 0.0000
Ns 200.0000
illum 3

newmtl fake_windsh
Ka 0.0000 0.0000 0.0000
Kd 0.0000 0.0000 0.0000
Ks 0.9000 0.9000 0.9000
d 0.1000
Ns 200
illum 4

# the rest of the example materials definitely won't work properly
";

        let materials = parse_mtl(input);

        let neon_green = materials.get("neon_green").unwrap();
        assert_eq!(
            neon_green.kind,
            MaterialKind::Solid(Colour::new(0.0, 1.0, 0.0))
        );
        assert_eq!(neon_green.ambient, 1.0);
        assert_eq!(neon_green.diffuse, 0.0);
        assert_eq!(neon_green.specular, 0.0);

        let flat_green = materials.get("flat_green").unwrap();
        assert_eq!(
            flat_green.kind,
            MaterialKind::Solid(Colour::new(0.0, 1.0, 0.0))
        );
        assert_eq!(flat_green.ambient, 0.1);
        assert_eq!(flat_green.specular, 0.0);
        assert_eq!(flat_green.diffuse, 0.9);

        let diss_green = materials.get("diss_green").unwrap();
        assert_eq!(
            diss_green.kind,
            MaterialKind::Solid(Colour::new(0.0, 1.0, 0.0))
        );
        assert_eq!(diss_green.ambient, 0.1);
        assert!(diss_green.transparency.roughly_equals(0.2));

        let shiny_green = materials.get("shiny_green").unwrap();
        assert_eq!(
            shiny_green.kind,
            MaterialKind::Solid(Colour::new(0.0, 1.0, 0.0))
        );
        assert_eq!(shiny_green.diffuse, 0.9);
        assert_eq!(shiny_green.specular, 1.0);
        assert_eq!(shiny_green.shininess, 200.0);

        let green_mirror = materials.get("green_mirror").unwrap();
        assert_eq!(green_mirror.ambient, 0.1);
        assert_eq!(
            green_mirror.kind,
            MaterialKind::Solid(Colour::new(0.0, 1.0, 0.0))
        );
        assert_eq!(green_mirror.specular, 1.0);
        assert_eq!(green_mirror.shininess, 200.0);
        assert_eq!(green_mirror.reflective, 1.0);

        // this probably isn't right, but it's the best it's getting
        let fake_windshield = materials.get("fake_windsh").unwrap();
        assert_eq!(fake_windshield.ambient, 0.0);
        assert_eq!(fake_windshield.kind, MaterialKind::Solid(Colour::BLACK));
        assert_eq!(fake_windshield.specular, 0.9);
        assert_eq!(fake_windshield.transparency, 0.9);
        assert_eq!(fake_windshield.shininess, 200.0);
        assert_eq!(fake_windshield.reflective, 1.0);
    }
}

mod obj_parser_tests {
    use super::*;

    trait VerticesExt {
        fn vertices(&self) -> Vec<usize>;
    }

    impl VerticesExt for Vec<PolygonData> {
        fn vertices(&self) -> Vec<usize> {
            self.iter().map(|p| p.vertex).collect::<Vec<_>>()
        }
    }

    #[test]
    fn parser_should_ignore_unrecognised_lines() {
        let invalid_obj_file = "There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.";

        let out = parse_obj(invalid_obj_file);
        assert!(out.vertices.is_empty());
        assert!(out.normals.is_empty());
        assert!(out.groups.is_empty());
    }

    #[test]
    fn parser_should_parse_vertex_data() {
        let input = "v -1 1 0
        v -1.0000 0.5000 0.0000
        v 1 0 0
        v 1 1 0";

        let out = parse_obj(input);
        assert_eq!(out.vertex(1), Some(Point3D::new(-1.0, 1.0, 0.0)));
        assert_eq!(out.vertex(2), Some(Point3D::new(-1.0, 0.5, 0.0)));
        assert_eq!(out.vertex(3), Some(Point3D::new(1.0, 0.0, 0.0)));
        assert_eq!(out.vertex(4), Some(Point3D::new(1.0, 1.0, 0.0)));
    }

    #[test]
    fn parser_should_parse_face_data() {
        let input = "v -1 1 0
        v 1 0 0
        v 1 0 0
        v 1 1 0
        
        f 1 2 3
        f 1 3 4";

        let out = parse_obj(input);
        assert_eq!(out.groups[0][0].vertices(), vec![1, 2, 3]);
        assert_eq!(out.groups[0][1].vertices(), vec![1, 3, 4]);
    }

    #[test]
    fn parser_should_parse_polygons_with_more_than_3_vertices() {
        let input = "v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5";

        let out = parse_obj(input);
        assert_eq!(out.groups[0][0].vertices(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn obj_data_should_be_convertible_to_group_containing_parsed_faces() {
        let input = "v -1 1 0
        v 1 0 0
        v 1 0 0
        v 1 1 0
        
        f 1 2 3
        f 1 3 4";

        let out = parse_obj(input);
        let object = out.to_object();
        assert!(object.is_ok(), "{}", object.unwrap_err());
        let object = object.unwrap();

        // this is moderately disgusting, but Shapes are totally opaque at runtime, so this seems to be the least worst way to introspect the fields
        assert_eq!(
            format!("{:?}", object.children()[0].shape()),
            "Triangle { \
            p1: Point3D(-1.0, 1.0, 0.0), \
            p2: Point3D(1.0, 0.0, 0.0), \
            p3: Point3D(1.0, 0.0, 0.0), \
            edge1: Vector3D(2.0, -1.0, 0.0), \
            edge2: Vector3D(2.0, -1.0, 0.0), \
            kind: Uniform(Normal3D(0.0, 0.0, 0.0)) \
            }"
            .to_string()
        );

        assert_eq!(
            format!("{:?}", object.children()[1].shape()),
            "Triangle { \
            p1: Point3D(-1.0, 1.0, 0.0), \
            p2: Point3D(1.0, 0.0, 0.0), \
            p3: Point3D(1.0, 1.0, 0.0), \
            edge1: Vector3D(2.0, -1.0, 0.0), \
            edge2: Vector3D(2.0, 0.0, 0.0), \
            kind: Uniform(Normal3D(0.0, 0.0, -1.0)) \
            }"
            .to_string()
        );
    }

    #[test]
    fn converting_to_group_should_triangulate_polygon_faces() {
        let input = "v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5";

        let out = parse_obj(input);
        let object = out.to_object();
        assert!(object.is_ok(), "{}", object.unwrap_err());
        let object = object.unwrap();

        assert_eq!(
            format!("{:?}", object.children()[0].shape()),
            "Triangle { \
            p1: Point3D(-1.0, 1.0, 0.0), \
            p2: Point3D(-1.0, 0.0, 0.0), \
            p3: Point3D(1.0, 0.0, 0.0), \
            edge1: Vector3D(0.0, -1.0, 0.0), \
            edge2: Vector3D(2.0, -1.0, 0.0), \
            kind: Uniform(Normal3D(0.0, 0.0, -1.0)) \
            }"
            .to_string()
        );

        assert_eq!(
            format!("{:?}", object.children()[1].shape()),
            "Triangle { \
            p1: Point3D(-1.0, 1.0, 0.0), \
            p2: Point3D(1.0, 0.0, 0.0), \
            p3: Point3D(1.0, 1.0, 0.0), \
            edge1: Vector3D(2.0, -1.0, 0.0), \
            edge2: Vector3D(2.0, 0.0, 0.0), \
            kind: Uniform(Normal3D(0.0, 0.0, -1.0)) \
            }"
            .to_string()
        );

        assert_eq!(
            format!("{:?}", object.children()[2].shape()),
            "Triangle { \
             p1: Point3D(-1.0, 1.0, 0.0), \
             p2: Point3D(1.0, 1.0, 0.0), \
             p3: Point3D(0.0, 2.0, 0.0), \
             edge1: Vector3D(2.0, 0.0, 0.0), \
             edge2: Vector3D(1.0, 1.0, 0.0), \
             kind: Uniform(Normal3D(0.0, 0.0, -1.0)) \
             }"
            .to_string()
        );
    }

    #[test]
    fn obj_parser_should_preserve_named_groups() {
        let input = "v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        
        g FirstGroup
        f 1 2 3
        g SecondGroup
        f 1 3 4";

        let output = parse_obj(input);
        assert_eq!(output.groups[0][0].vertices(), vec![1, 2, 3]);
        assert_eq!(output.groups[1][0].vertices(), vec![1, 3, 4]);
    }

    #[test]
    fn converting_obj_data_with_multiple_groups_should_create_a_group_with_subgroups() {
        let input = "v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        
        g FirstGroup
        f 1 2 3
        g SecondGroup
        f 1 3 4";

        let output = parse_obj(input);
        let object = output.to_object();
        assert!(object.is_ok(), "{}", object.unwrap_err());
        let object = object.unwrap();

        assert_eq!(object.children().len(), 2);
        assert_eq!(object.children()[0].children().len(), 1);
        assert_eq!(object.children()[1].children().len(), 1);
    }

    #[test]
    fn obj_parser_should_parse_vertex_normals() {
        let input = "vn 0 0 1
        vn 0.707 0 -0.707
        vn 1 2 3";

        let output = parse_obj(input);
        assert_eq!(output.normals[0], Vector3D::new(0.0, 0.0, 1.0));
        assert_eq!(output.normals[1], Vector3D::new(0.707, 0.0, -0.707));
        assert_eq!(output.normals[2], Vector3D::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn obj_parser_should_parse_faces_with_texture_and_normal_indexes() {
        let input = "v 0 1 0
        v -1 0 0
        v 1 0 0
        
        vn -1 0 0
        vn 1 0 0
        vn 0 1 0
        
        f 1//3 2//1 3//2
        f 1/0/3 2/102/1 3/14/2";

        let output = parse_obj(input);
        assert_eq!(
            output.groups[0][0][0],
            PolygonData {
                vertex: 1,
                texture_vertex: None,
                normal: Some(3)
            }
        );
        assert_eq!(
            output.groups[0][0][1],
            PolygonData {
                vertex: 2,
                texture_vertex: None,
                normal: Some(1)
            }
        );
        assert_eq!(
            output.groups[0][0][2],
            PolygonData {
                vertex: 3,
                texture_vertex: None,
                normal: Some(2)
            }
        );

        assert_eq!(
            output.groups[0][1][0],
            PolygonData {
                vertex: 1,
                texture_vertex: Some(0),
                normal: Some(3)
            }
        );
        assert_eq!(
            output.groups[0][1][1],
            PolygonData {
                vertex: 2,
                texture_vertex: Some(102),
                normal: Some(1)
            }
        );
        assert_eq!(
            output.groups[0][1][2],
            PolygonData {
                vertex: 3,
                texture_vertex: Some(14),
                normal: Some(2)
            }
        );
    }

    #[test]
    fn converting_obj_data_should_convert_faces_with_normals_into_smooth_triangles() {
        let input = "v 0 1 0
        v -1 0 0
        v 1 0 0
        
        vn -1 0 0
        vn 1 0 0
        vn 0 1 0
        
        f 1//3 2//1 3//2
        f 1/0/3 2/102/1 3/14/2";

        let output = parse_obj(input);
        let object = output.to_object();
        assert!(object.is_ok(), "{}", object.unwrap_err());
        let object = object.unwrap();

        assert_eq!(
            format!("{:?}", object.children()[0].shape()),
            "Triangle { \
            p1: Point3D(0.0, 1.0, 0.0), \
            p2: Point3D(-1.0, 0.0, 0.0), \
            p3: Point3D(1.0, 0.0, 0.0), \
            edge1: Vector3D(-1.0, -1.0, 0.0), \
            edge2: Vector3D(1.0, -1.0, 0.0), \
            kind: Smooth { \
            normal1: Normal3D(0.0, 1.0, 0.0), \
            normal2: Normal3D(-1.0, 0.0, 0.0), \
            normal3: Normal3D(1.0, 0.0, 0.0) \
            } \
            }"
            .to_string()
        );

        assert_eq!(
            format!("{:?}", object.children()[1].shape()),
            "Triangle { \
            p1: Point3D(0.0, 1.0, 0.0), \
            p2: Point3D(-1.0, 0.0, 0.0), \
            p3: Point3D(1.0, 0.0, 0.0), \
            edge1: Vector3D(-1.0, -1.0, 0.0), \
            edge2: Vector3D(1.0, -1.0, 0.0), \
            kind: Smooth { \
            normal1: Normal3D(0.0, 1.0, 0.0), \
            normal2: Normal3D(-1.0, 0.0, 0.0), \
            normal3: Normal3D(1.0, 0.0, 0.0) \
            } \
            }"
            .to_string()
        );
    }
}
