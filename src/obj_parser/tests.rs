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
        assert_eq!(out.faces[0], (1, 2, 3));
        assert_eq!(out.faces[1], (1, 3, 4));
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
    }
}
