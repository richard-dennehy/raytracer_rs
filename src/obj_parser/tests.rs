use super::*;

mod unit_tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn parser_should_ignore_unrecognised_lines() {
        let invalid_obj_file = "There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.";

        let out = parse(invalid_obj_file);
        assert_eq!(out.ignored_lines, 5);
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
        assert_eq!(out.groups[0][0], vec![1, 2, 3]);
        assert_eq!(out.groups[0][1], vec![1, 3, 4]);
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
        assert_eq!(out.groups[0][0], vec![1, 2, 3, 4, 5]);
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

        assert_eq!(
            object.children()[0].vertices()[0],
            Point3D::new(-1.0, 1.0, 0.0)
        );
        assert_eq!(
            object.children()[0].vertices()[1],
            Point3D::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            object.children()[0].vertices()[2],
            Point3D::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            object.children()[1].vertices()[0],
            Point3D::new(-1.0, 1.0, 0.0)
        );
        assert_eq!(
            object.children()[1].vertices()[1],
            Point3D::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            object.children()[1].vertices()[2],
            Point3D::new(1.0, 1.0, 0.0)
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
            object.children()[0].vertices()[0],
            Point3D::new(-1.0, 1.0, 0.0)
        );
        assert_eq!(
            object.children()[0].vertices()[1],
            Point3D::new(-1.0, 0.0, 0.0)
        );
        assert_eq!(
            object.children()[0].vertices()[2],
            Point3D::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            object.children()[1].vertices()[0],
            Point3D::new(-1.0, 1.0, 0.0)
        );
        assert_eq!(
            object.children()[1].vertices()[1],
            Point3D::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            object.children()[1].vertices()[2],
            Point3D::new(1.0, 1.0, 0.0)
        );

        assert_eq!(
            object.children()[2].vertices()[0],
            Point3D::new(-1.0, 1.0, 0.0)
        );
        assert_eq!(
            object.children()[2].vertices()[1],
            Point3D::new(1.0, 1.0, 0.0)
        );
        assert_eq!(
            object.children()[2].vertices()[2],
            Point3D::new(0.0, 2.0, 0.0)
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
        assert_eq!(output.groups[0][0], vec![1, 2, 3]);
        assert_eq!(output.groups[1][0], vec![1, 3, 4]);
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
}
