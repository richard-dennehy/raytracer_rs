use ray_tracer::core::{Colour, Normal3D, Point3D, Transform};
use ray_tracer::renderer::Samples;
use ray_tracer::scene::{Light, Material, MaterialKind, Object, Pattern, World};
use ray_tracer::{image_writer, renderer, Camera};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/csgs");
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(5.0, 10.0, -10.0)));

    world.add(
        Object::plane()
            .with_material(Material {
                kind: MaterialKind::Pattern(Pattern::checkers(
                    Colour::greyscale(0.1),
                    Colour::greyscale(0.8),
                )),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_y(-1.0)),
    );
    world.add(
        Object::plane().transformed(Transform::identity().rotate_x(FRAC_PI_2).translate_z(5.0)),
    );

    world.add(
        Object::csg_difference(
            Object::cube().with_material(Material {
                kind: MaterialKind::Solid(Colour::RED),
                ..Default::default()
            }),
            Object::sphere()
                .with_material(Material {
                    kind: MaterialKind::Solid(Colour::new(1.0, 1.0, 0.0)),
                    ..Default::default()
                })
                .transformed(
                    Transform::identity()
                        .scale_all(1.2)
                        .translate_x(1.0)
                        .translate_z(-1.0)
                        .translate_y(1.0),
                ),
        )
        .transformed(Transform::identity().translate_x(-4.5)),
    );

    fn transparent_sphere(colour: f64) -> Object {
        Object::sphere().with_material(Material {
            kind: MaterialKind::Solid(Colour::greyscale(colour)),
            transparency: 0.7,
            ..Default::default()
        })
    }

    world.add(
        Object::csg_union(
            transparent_sphere(0.0),
            transparent_sphere(0.2).transformed(Transform::identity().translate_y(1.0)),
        )
        .transformed(Transform::identity().translate_x(-1.5)),
    );

    world.add(
        Object::csg_intersection(
            Object::sphere()
                .with_material(Material {
                    kind: MaterialKind::Solid(Colour::new(0.0, 1.0, 1.0)),
                    ..Default::default()
                })
                .transformed(Transform::identity().scale_all(1.3)),
            Object::cube().with_material(Material {
                kind: MaterialKind::Solid(Colour::BLUE),
                ..Default::default()
            }),
        )
        .transformed(Transform::identity().translate_x(1.5)),
    );

    fn short_cylinder(colour: Colour) -> Object {
        Object::cylinder()
            .min_y(-1.1)
            .max_y(1.1)
            .capped()
            .build()
            .with_material(Material {
                kind: MaterialKind::Solid(colour),
                ..Default::default()
            })
            .transformed(Transform::identity().scale_x(0.4).scale_z(0.4))
    }

    world.add(
        Object::csg_difference(
            Object::csg_intersection(
                Object::cube().with_material(Material {
                    kind: MaterialKind::Solid(Colour::BLACK),
                    reflective: 0.6,
                    ..Default::default()
                }),
                Object::sphere()
                    .with_material(Material {
                        kind: MaterialKind::Solid(Colour::BLACK),
                        ..Default::default()
                    })
                    .transformed(Transform::identity().scale_all(1.4)),
            ),
            Object::csg_union(
                Object::csg_union(
                    short_cylinder(Colour::RED),
                    short_cylinder(Colour::BLUE)
                        .transformed(Transform::identity().rotate_x(FRAC_PI_2)),
                ),
                short_cylinder(Colour::GREEN)
                    .transformed(Transform::identity().rotate_z(FRAC_PI_2)),
            ),
        )
        .transformed(Transform::identity().rotate_y(FRAC_PI_6).translate_x(4.5)),
    );

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_3,
        Transform::view_transform(
            Point3D::new(0.0, 3.0, -12.0),
            Point3D::ORIGIN,
            Normal3D::POSITIVE_Y,
        ),
    );

    let canvas = renderer::render(&world, &camera, &Samples::single(), true);

    let image = image_writer::write(canvas);
    image
        .save(root.join("csgs.png"))
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
