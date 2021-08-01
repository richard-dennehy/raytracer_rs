use super::*;
use crate::core::Colour;
use std::num::NonZeroU16;

#[test]
fn a_canvas_should_have_a_valid_height_and_width() {
    let canvas = Canvas::new(NonZeroU16::new(10).unwrap(), NonZeroU16::new(20).unwrap());

    assert!(canvas.is_some());
    let canvas = canvas.unwrap();

    assert_eq!(canvas.width(), 10);
    assert_eq!(canvas.height(), 20);
}

#[test]
fn creating_a_canvas_should_set_all_pixels_to_black() {
    let canvas = Canvas::new(NonZeroU16::new(10).unwrap(), NonZeroU16::new(20).unwrap());

    assert!(canvas.is_some());
    let canvas = canvas.unwrap();

    for i in 0..10 {
        for j in 0..20 {
            assert_eq!(canvas.get(i, j), Colour::BLACK);
        }
    }
}

#[test]
#[cfg_attr(not(feature = "slow_tests"), ignore)] // iterating over ~20 million elements is slow
fn should_be_able_to_create_16k_canvas() {
    let canvas = Canvas::new(
        NonZeroU16::new(7680).unwrap(),
        NonZeroU16::new(4320).unwrap(),
    );

    assert!(canvas.is_some());
    let canvas = canvas.unwrap();

    assert_eq!(canvas.width(), 7680);
    assert_eq!(canvas.height(), 4320);

    for i in 0..7680 {
        for j in 0..4320 {
            assert_eq!(canvas.get(i, j), Colour::BLACK);
        }
    }
}

#[test]
fn should_not_be_able_to_create_oversized_canvas() {
    let canvas = Canvas::new(
        NonZeroU16::new(u16::MAX).unwrap(),
        NonZeroU16::new(u16::MAX).unwrap(),
    );

    assert!(canvas.is_none())
}

#[test]
fn should_be_able_to_set_pixel_colour() {
    let canvas = Canvas::new(NonZeroU16::new(10).unwrap(), NonZeroU16::new(20).unwrap());
    assert!(canvas.is_some());
    let mut canvas = canvas.unwrap();

    assert_eq!(canvas.get(2, 3), Colour::BLACK);
    canvas.set(2, 3, Colour::RED);
    assert_eq!(canvas.get(2, 3), Colour::RED);
}
