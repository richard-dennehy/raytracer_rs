use ray_tracer::renderer::Samples;
use ray_tracer::{
    image_writer, renderer, Camera, Colour, Light, Material, MaterialKind, Normal3D, Object,
    Point3D, Transform, UvPattern, World,
};
use std::f64::consts::PI;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

fn main() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/textured_earth");
    let timer = Instant::now();

    let mut world = World::empty();
    world
        .lights
        .push(Light::point(Colour::WHITE, Point3D::new(6.0, 10.0, -10.0)));

    let texture = Arc::new(
        image::open(root.join("textures/earthmap1k.jpg"))
            .unwrap()
            .to_rgb8(),
    );

    world.add(
        Object::sphere()
            .with_material(Material {
                kind: MaterialKind::Uv(UvPattern::image(texture)),
                ..Default::default()
            })
            .transformed(Transform::identity().rotate_y(PI)),
    );

    let camera = Camera::new(
        nonzero_ext::nonzero!(1920u16),
        nonzero_ext::nonzero!(1080u16),
        PI / 3.0,
        Transform::view_transform(
            Point3D::new(0.0, 0.0, -5.0),
            Point3D::ORIGIN,
            Normal3D::POSITIVE_Y,
        ),
    );
    let canvas = renderer::render(world, camera, &Samples::single());

    let image = image_writer::write(canvas);
    image
        .save(root.join("textured_earth.png"))
        .expect("failed to write output file");

    println!("Completed at {:.2?}", timer.elapsed());

    Ok(())
}
