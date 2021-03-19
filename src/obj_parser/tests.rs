use super::*;

mod unit_tests {
    use super::*;
    use std::convert::TryInto;

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

        let out = parse(invalid_obj_file);
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

        let out = parse(input);
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

        let out = parse(input);
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

        let out = parse(input);
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

        let out = parse(input);
        let object: Result<Object, _> = out.try_into();
        assert!(object.is_ok(), object.unwrap_err());
        let object = object.unwrap();

        // this is moderately disgusting, but Shapes are totally opaque at runtime, so this seems to be the least worst way to introspect the fields
        assert_eq!(
            format!("{:?}", object.children()[0].shape()),
            "Triangle { \
            p1: Point3D(-1.0, 1.0, 0.0), \
            p2: Point3D(1.0, 0.0, 0.0), \
            p3: Point3D(1.0, 0.0, 0.0), \
            edge1: Vector3D(2.0, -1.0, 0.0), \
            edge2: Vector3D(0.0, 0.0, 0.0), \
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
            edge2: Vector3D(0.0, 1.0, 0.0), \
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

        let out = parse(input);
        let object: Result<Object, _> = out.try_into();
        assert!(object.is_ok(), object.unwrap_err());
        let object = object.unwrap();

        assert_eq!(
            format!("{:?}", object.children()[0].shape()),
            "Triangle { \
            p1: Point3D(-1.0, 1.0, 0.0), \
            p2: Point3D(-1.0, 0.0, 0.0), \
            p3: Point3D(1.0, 0.0, 0.0), \
            edge1: Vector3D(0.0, -1.0, 0.0), \
            edge2: Vector3D(2.0, 0.0, 0.0), \
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
            edge2: Vector3D(0.0, 1.0, 0.0), \
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
             edge2: Vector3D(-1.0, 1.0, 0.0), \
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

        let output = parse(input);
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

        let output = parse(input);
        let object: Result<Object, _> = output.try_into();
        assert!(object.is_ok(), object.unwrap_err());
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

        let output = parse(input);
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

        let output = parse(input);
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

        let output = parse(input);
        let object: Result<Object, _> = output.try_into();
        assert!(object.is_ok(), object.unwrap_err());
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
