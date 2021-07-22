use ray_tracer::renderer::Samples;
use ray_tracer::wavefront_parser::WavefrontParser;
use ray_tracer::{
    image_writer, renderer, Camera, Colour, Light, Material, MaterialKind, Normal3D, Object,
    Pattern, Point3D, Transform, World,
};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_6, PI};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/meshes");
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(4.0, 11.0, -5.0)));

    world.add(
        Object::plane()
            .with_material(Material {
                kind: MaterialKind::Pattern(Pattern::checkers(
                    Colour::greyscale(0.2),
                    Colour::greyscale(0.8),
                )),
                ..Default::default()
            })
            .transformed(Transform::identity().translate_y(-1.0)),
    );

    world.add(
        Object::plane()
            .with_material(Material {
                kind: MaterialKind::Solid(Colour::BLACK),
                reflective: 0.9,
                ..Default::default()
            })
            .transformed(Transform::identity().rotate_x(FRAC_PI_2).translate_z(5.0)),
    );

    world.add(
        Object::plane()
            .with_material(Material {
                kind: MaterialKind::Solid(Colour::new(0.3, 0.3, 1.0)),
                ..Default::default()
            })
            .transformed(Transform::identity().rotate_x(FRAC_PI_2).translate_z(-20.0)),
    );

    let parser = WavefrontParser::new(root.join("resources"));
    let prism = parser.load("multicoloured prism.obj")?;
    let suzanne = parser.load("suzanne high poly.obj")?;

    world.add(prism.transformed(Transform::identity().translate_x(3.0).translate_y(-1.0)));
    world.add(
        suzanne.transformed(
            Transform::identity()
                .rotate_y(PI)
                .rotate_x(FRAC_PI_6)
                .translate_x(-3.0)
                .translate_y(-0.4),
        ),
    );

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        FRAC_PI_3,
        Transform::view_transform(
            Point3D::new(0.0, 2.0, -10.0),
            Point3D::ORIGIN,
            Normal3D::POSITIVE_Y,
        ),
    );
    let canvas = renderer::render(world, camera, &Samples::single());

    let image = image_writer::write(canvas);
    image
        .save(root.join("meshes.png"))
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
