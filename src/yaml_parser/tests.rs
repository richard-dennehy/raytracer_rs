use super::*;

mod unit_tests {
    use super::*;
    use crate::Vector3D;

    #[test]
    fn should_parse_camera_description() {
        let input = "\
# ======================================================
# the camera
# ======================================================

- add: camera
  width: 100
  height: 100
  field-of-view: 0.785
  from: [ -6, 6, -10 ]
  to: [ 6, 0, 6 ]
  up: [ -0.45, 1, 0 ]";

        let output = parse(input);
        assert!(output.is_ok(), output.unwrap_err());
        let output = output.unwrap();

        assert_eq!(output, CameraDescription {
            width: 100,
            height: 100,
            field_of_view: 0.785,
            from: Point3D::new(-6.0, 6.0, -10.0),
            to: Point3D::new(6.0, 0.0, 6.0),
            up: Vector3D::new(-0.45, 1.0, 0.0),
        })
    }
}
