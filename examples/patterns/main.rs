use ray_tracer::core::{Colour, Normal3D, Point3D, Transform};
use ray_tracer::renderer::{Camera, Samples};
use ray_tracer::scene::{Light, Material, MaterialKind, Object, Pattern, World};
use ray_tracer::{image_writer, renderer};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/patterns");
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(5.0, 10.0, -10.0)));

    world.add(Object::plane().transformed(Transform::identity().translate_y(-1.0)));
    world.add(
        Object::plane().transformed(Transform::identity().rotate_x(FRAC_PI_2).translate_z(5.0)),
    );

    world.add(
        Object::cube()
            .with_material(Material {
                kind: MaterialKind::Pattern(
                    Pattern::striped(Colour::RED, Colour::WHITE)
                        .with_transform(Transform::identity().scale_all(0.25).rotate_z(FRAC_PI_2)),
                ),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_x(-4.5)),
    );

    world.add(
        Object::sphere()
            .with_material(Material {
                kind: MaterialKind::Pattern(Pattern::gradient(
                    Colour::new(1.0, 0.5, 0.0),
                    Colour::new(1.0, 1.0, 0.0),
                )),
                ..Default::default()
            })
            .transformed(Transform::identity().rotate_z(FRAC_PI_4).translate_x(-1.5)),
    );

    world.add(
        Object::cube()
            .with_material(Material {
                kind: MaterialKind::Pattern(
                    Pattern::ring(Colour::GREEN, Colour::new(0.0, 0.7, 0.0))
                        .with_transform(Transform::identity().scale_all(0.25)),
                ),
                ..Default::default()
            })
            .transformed(Transform::identity().rotate_x(FRAC_PI_2).translate_x(1.5)),
    );

    world.add(
        Object::cube()
            .with_material(Material {
                kind: MaterialKind::Pattern(Pattern::checkers(
                    Colour::BLUE,
                    Colour::new(0.0, 1.0, 1.0),
                )),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_x(4.5)),
    );

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_3,
        Transform::view_transform(
            Point3D::new(-1.0, 2.0, -12.0),
            Point3D::ORIGIN,
            Normal3D::POSITIVE_Y,
        ),
    );
    let canvas = renderer::render(&world, &camera, &Samples::single(), true);

    let image = image_writer::write(canvas);
    image
        .save(root.join("patterns.png"))
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
