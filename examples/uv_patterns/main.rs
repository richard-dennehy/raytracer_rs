use ray_tracer::core::{Colour, Normal3D, Point3D, Transform};
use ray_tracer::renderer::{Camera, Samples};
use ray_tracer::scene::{Light, Material, MaterialKind, Object, UvPattern, World};
use ray_tracer::{image_writer, renderer};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/uv_patterns");
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(5.0, 10.0, -10.0)));

    world.add(Object::plane().transformed(Transform::identity().translate_y(-1.0)));
    world.add(
        Object::plane().transformed(Transform::identity().rotate_x(FRAC_PI_2).translate_z(5.0)),
    );

    fn green_white_checkers() -> UvPattern {
        UvPattern::checkers(
            Colour::new(0.0, 0.5, 0.0),
            Colour::WHITE,
            nonzero_ext::nonzero!(8usize),
            nonzero_ext::nonzero!(8usize),
        )
    }

    world.add(
        Object::cube()
            .with_material(Material {
                kind: MaterialKind::Uv(UvPattern::cubic(
                    green_white_checkers(),
                    green_white_checkers(),
                    green_white_checkers(),
                    green_white_checkers(),
                    green_white_checkers(),
                    green_white_checkers(),
                )),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_x(-5.0)),
    );

    world.add(
        Object::cube()
            .with_material(Material {
                kind: MaterialKind::Uv(UvPattern::cubic(
                    UvPattern::alignment_check(
                        Colour::new(0.0, 1.0, 1.0),
                        Colour::RED,
                        Colour::new(1.0, 1.0, 0.0),
                        Colour::new(1.0, 0.5, 0.0),
                        Colour::GREEN,
                    ),
                    UvPattern::alignment_check(
                        Colour::GREEN,
                        Colour::new(1.0, 0.0, 1.0),
                        Colour::new(0.0, 1.0, 1.0),
                        Colour::WHITE,
                        Colour::BLUE,
                    ),
                    UvPattern::alignment_check(
                        Colour::new(1.0, 1.0, 0.0),
                        Colour::new(0.0, 1.0, 1.0),
                        Colour::RED,
                        Colour::BLUE,
                        Colour::new(1.0, 0.5, 0.0),
                    ),
                    UvPattern::alignment_check(
                        Colour::RED,
                        Colour::new(1.0, 1.0, 0.0),
                        Colour::new(1.0, 0.0, 1.0),
                        Colour::GREEN,
                        Colour::WHITE,
                    ),
                    UvPattern::alignment_check(
                        Colour::new(1.0, 0.5, 0.0),
                        Colour::new(0.0, 1.0, 1.0),
                        Colour::new(1.0, 0.0, 1.0),
                        Colour::RED,
                        Colour::new(1.0, 1.0, 0.0),
                    ),
                    UvPattern::alignment_check(
                        Colour::new(1.0, 0.0, 1.0),
                        Colour::new(1.0, 0.5, 0.0),
                        Colour::GREEN,
                        Colour::BLUE,
                        Colour::WHITE,
                    ),
                )),
                ..Default::default()
            })
            .transformed(Transform::identity().rotate_y(FRAC_PI_6).translate_x(-1.5)),
    );

    world.add(
        Object::sphere()
            .with_material(Material {
                kind: MaterialKind::Uv(UvPattern::checkers(
                    Colour::new(0.0, 0.5, 0.0),
                    Colour::WHITE,
                    nonzero_ext::nonzero!(16usize),
                    nonzero_ext::nonzero!(8usize),
                )),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_x(1.5)),
    );

    world.add(
        Object::cylinder()
            .min_y(0.0)
            .max_y(1.0)
            .capped()
            .build()
            .with_material(Material {
                kind: MaterialKind::Uv(UvPattern::checkers(
                    Colour::new(0.0, 0.5, 0.0),
                    Colour::WHITE,
                    nonzero_ext::nonzero!(16usize),
                    nonzero_ext::nonzero!(8usize),
                )),
                ..Default::default()
            })
            .transformed(
                Transform::identity()
                    .scale_y(2.0)
                    .translate_y(-1.0)
                    .translate_x(4.5),
            ),
    );

    world.add(
        Object::cone()
            .min_y(0.0)
            .max_y(2.0)
            .capped()
            .build()
            .with_material(Material {
                kind: MaterialKind::Uv(UvPattern::checkers(
                    Colour::new(0.0, 0.5, 0.0),
                    Colour::WHITE,
                    nonzero_ext::nonzero!(8usize),
                    nonzero_ext::nonzero!(4usize),
                )),
                ..Default::default()
            })
            .transformed(
                Transform::identity()
                    .scale_x(0.5)
                    .scale_z(0.5)
                    .translate_y(-1.0)
                    .translate_x(7.5),
            ),
    );

    // TODO add triangles

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_3,
        Transform::view_transform(
            Point3D::new(2.0, 4.0, -14.0),
            Point3D::new(1.0, 0.0, 0.0),
            Normal3D::POSITIVE_Y,
        ),
    );
    let canvas = renderer::render(&world, &camera, &Samples::single(), true);

    let image = image_writer::write(canvas);
    image
        .save(root.join("uv_patterns.png"))
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
