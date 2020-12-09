use super::*;

mod unit_tests {
    use super::*;

    #[test]
    fn should_be_able_to_create_4d_matrix() {
        let matrix4d = Matrix4D::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        );

        assert_eq!(matrix4d.m00(), 1.0);
        assert_eq!(matrix4d.m03(), 4.0);
        assert_eq!(matrix4d.m10(), 5.5);
        assert_eq!(matrix4d.m12(), 7.5);
        assert_eq!(matrix4d.m22(), 11.0);
        assert_eq!(matrix4d.m30(), 13.5);
        assert_eq!(matrix4d.m32(), 15.5);
    }

    #[test]
    fn should_be_able_to_create_3d_matrix() {
        #[rustfmt::skip]
        let matrix3d = Matrix3D::new(
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0], 
            [0.0, 1.0, 1.0]
        );

        assert_eq!(matrix3d.m00(), -3.0);
        assert_eq!(matrix3d.m11(), -2.0);
        assert_eq!(matrix3d.m22(), 1.0);
    }

    #[test]
    fn should_be_able_to_create_2d_matrix() {
        #[rustfmt::skip]
        let matrix2d = Matrix2D::new(
            [-3.0, 5.0],
            [1.0, -2.0],
        );

        assert_eq!(matrix2d.m00(), -3.0);
        assert_eq!(matrix2d.m01(), 5.0);
        assert_eq!(matrix2d.m10(), 1.0);
        assert_eq!(matrix2d.m11(), -2.0);
    }
}
