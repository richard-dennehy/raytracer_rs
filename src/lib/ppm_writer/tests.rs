use super::*;

mod unit_tests {
    use super::*;
    use crate::Colour;
    use std::num::NonZeroU16;

    #[test]
    fn should_generate_correct_header() {
        let ppm = write_ppm(
            &Canvas::new(NonZeroU16::new(5).unwrap(), NonZeroU16::new(3).unwrap()).unwrap(),
        );

        let header = ppm
            .lines()
            .take(3)
            .map(|line| format!("{}\n", line))
            .collect::<String>();

        assert_eq!(header, "P3\n5 3\n255\n");
    }

    #[test]
    fn should_generate_correct_pixel_data() {
        let mut canvas =
            Canvas::new(NonZeroU16::new(5).unwrap(), NonZeroU16::new(3).unwrap()).unwrap();
        canvas.set(0, 0, Colour::new(1.5, 0.0, 0.0));
        canvas.set(2, 1, Colour::new(0.0, 0.5, 0.0));
        canvas.set(4, 2, Colour::new(-0.5, 0.0, 1.0));

        let ppm = write_ppm(&canvas);

        let pixel_data = ppm
            .lines()
            .skip(3)
            .map(|line| format!("{}\n", line))
            .collect::<String>();

        assert_eq!(
            pixel_data,
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"
        );
    }

    #[test]
    fn ppm_colour_value_should_clamp_negative_colour_values_to_0() {
        assert_eq!(ppm_colour_value(-1.0), 0);
    }

    #[test]
    fn ppm_colour_value_should_clamp_colour_values_greater_than_1_f64_to_255() {
        assert_eq!(ppm_colour_value(2.0), MAX_COLOUR_VALUE);
    }

    #[test]
    fn ppm_colour_value_should_convert_1_f64_to_255() {
        assert_eq!(ppm_colour_value(1.0), MAX_COLOUR_VALUE);
    }

    #[test]
    fn ppm_colour_value_should_convert_0_f64_to_0() {
        assert_eq!(ppm_colour_value(0.0), 0);
    }

    #[test]
    fn ppm_colour_value_should_convert_0_5_f64_to_128() {
        assert_eq!(ppm_colour_value(0.5), 128);
    }

    #[test]
    fn should_limit_line_length_to_70() {
        let mut canvas =
            Canvas::new(NonZeroU16::new(10).unwrap(), NonZeroU16::new(2).unwrap()).unwrap();
        for x in 0..10 {
            for y in 0..2 {
                canvas.set(x, y, Colour::new(1.0, 0.8, 0.6))
            }
        }

        let ppm = write_ppm(&canvas);
        let pixel_data = ppm
            .lines()
            .skip(3)
            .map(|line| format!("{}\n", line))
            .collect::<String>();

        assert_eq!(
            pixel_data,
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"
        );
    }

    #[test]
    fn should_end_ppm_with_newline() {
        let ppm = write_ppm(
            &Canvas::new(NonZeroU16::new(5).unwrap(), NonZeroU16::new(3).unwrap()).unwrap(),
        );

        assert_eq!(ppm.chars().last(), Some('\n'))
    }
}
