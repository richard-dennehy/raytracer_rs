use ray_tracer::renderer::Samples;
use ray_tracer::{
    image_writer, renderer, Camera, Colour, Light, Material, MaterialKind, Normal3D, Object,
    Pattern, Point3D, Transform, World,
};
use std::f64::consts::{FRAC_PI_3, PI};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/primitives");
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(10.0, 10.0, -10.0)));

    world.add(
        Object::plane()
            .with_material(Material {
                kind: MaterialKind::Pattern(Pattern::checkers(Colour::BLACK, Colour::WHITE)),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_y(-1.0)),
    );

    world.add(
        Object::plane().transformed(Transform::identity().rotate_x(PI / 2.0).translate_z(10.0)),
    );

    world.add(
        Object::sphere()
            .with_material(Material {
                kind: MaterialKind::Solid(Colour::RED),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_x(-6.0)),
    );

    world.add(
        Object::cube()
            .with_material(Material {
                kind: MaterialKind::Solid(Colour::GREEN),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_x(-3.0)),
    );

    world.add(
        Object::cylinder()
            .min_y(-1.0)
            .max_y(2.0)
            .capped()
            .build()
            .with_material(Material {
                kind: MaterialKind::Solid(Colour::BLUE),
                ..Default::default()
            })
            .transformed(Transform::identity()),
    );

    world.add(
        Object::cone()
            .min_y(-1.5)
            .max_y(0.0)
            .capped()
            .build()
            .with_material(Material {
                kind: MaterialKind::Solid(Colour::new(1.0, 1.0, 0.0)),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_y(0.5).translate_x(3.0)),
    );

    world.add(
        Object::triangle(
            Point3D::ORIGIN,
            Point3D::new(0.0, 2.0, 0.0),
            Point3D::new(2.0, 0.0, 0.0),
        )
        .with_material(Material {
            kind: MaterialKind::Solid(Colour::new(0.0, 1.0, 1.0)),
            ..Default::default()
        })
        .transformed(Transform::identity().translate_y(-1.0).translate_x(5.0)),
    );

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_3,
        Transform::view_transform(
            Point3D::new(0.0, 2.0, -14.0),
            Point3D::ORIGIN,
            Normal3D::POSITIVE_Y,
        ),
    );
    let canvas = renderer::render(&world, &camera, &Samples::single());

    let image = image_writer::write(canvas);
    image
        .save(root.join("primitives.png"))
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
