use yaml_rust::YamlLoader;

use model::*;
use parsers::*;

#[cfg(test)]
mod tests;

mod model;
mod parsers;

/// Given the YAML example provided doesn't come with a schema, and doesn't provide many hints for
/// a parser implementation to use (e.g. type tags, or any consistent structure), the implementation
/// of the parser loosely resembles an event-driven streaming parser, as there's no way to navigate
/// the structure without resorting to iterating over the values in the outer array, and then making
/// educated guesses at the structure by looking for the presence of known keys with known values.
///
/// Given the above limitations, the parser may behave very strangely when given unexpected input.
///
/// From the single example and some common sense, I've reverse-engineered something resembling a schema,
/// but with a number of limitations:
///
/// - The main structure is a list of objects
/// - The list must contain an object with an `add: camera` field (because it makes no sense to render a scene with no camera)
/// - The camera must contain a `width`, `height`, `field-of-view`, `from`, `to`, and `up` - there's not really sensible defaults for these
/// - 3-tuples i.e. `Colour`, `Point3D`, and `Vector3D` are described as arrays of numbers (Reals or Integers)
/// - Point lights are added using `add: light`
/// - Point lights must contain an `at` (position) and `intensity` (colour)
/// - Common definitions may be provided using a `define` object (an object that has `define` key)
/// - The value of the `define` key identifies it, and may be referenced by other defines or objects
/// - A `define` object with a `value` which is an object describes a material
/// - A `define` object with a `value` which is an array describes a sequence of transforms
/// - A material `define` may `extend` another material define
/// - A transform `define` may _reference_ another transform, but cannot `extend` one
/// - Materials may have any of the following fields: `color`, `diffuse`, `ambient`, `specular`, `shininess`, `reflective`, `transparency`, and `refractive-index` (`refraction`)
/// - A material that `extend`s another material will inherit any values that are defined by the extended material, but not by the child material
/// - Any missing fields on a material will be set to default values (see crate::material::Material)
/// - An entry in a transform `define` value array may either be a string referencing another transform define, or an array describing a transform
/// - A transform array's first field describes the transform type, and must be either `translate`, `scale`, `rotate-x`, `rotate-y`, `rotate-z`, or `shearing`
/// - A `translate` array must contain exactly 4 fields: `translate`, the `x` value, `y` value, and `z` value
/// - A `scale` array must contain exactly 4 fields: `scale`, the `x` value, `y` value, and `z` value
/// - A `rotate-x` array must contain exactly 2 fields: `rotate-x`, and the angle in radians
/// - A `rotate-y` array must contain exactly 2 fields: `rotate-y`, and the angle in radians
/// - A `rotate-z` array must contain exactly 2 fields: `rotate-z`, and the angle in radians
/// - No `shearing` transform is used in the example - this name matches the name in the book, but may be wrong
/// - Following the example of the other transforms, a `shearing` array must contain exactly 7 fields:
///     - `shearing`
///     - `x` in proportion to `y`
///     - `x` in proportion to `z`
///     - `y` in proportion to `x`
///     - `y` in proportion to `z`
///     - `z` in proportion to `x`
///     - `z` in proportion to `y`
/// - Note: the identity value for a shear is 0
/// - Note: a shear with a non-zero e.g. x to y and y to x is not invertible, and therefore cannot be used
/// - Lists of transforms are combined into a single transform which effectively applies each transform in sequence, i.e.:
///     ```yaml
///       - [ translate, 1, -1, 1 ]
///       - [ scale, 0.5, 0.5, 0.5 ]        
///     ```
///   will translate, then scale
/// - Objects may be added to the scene using an `add` - the object added depends on the value of `add`
/// - Objects that may be added are `plane`, `cube`, `sphere`, `cylinder`, and `cone` - triangles are not supported
/// - An object must have a `material` and a `transform`
/// - An object material may be a string referencing a define, or a material definition as described above
/// - An object's transforms must be an array of transforms, as described above
/// - To effectively apply the identity matrix instead (i.e. no transform), use an empty array `[]`
pub fn parse(input: &str) -> Result<SceneDescription, String> {
    match YamlLoader::load_from_str(input) {
        Ok(yaml) => {
            let mut camera = None;
            let mut lights = vec![];
            let mut defines = vec![];
            let mut objects = vec![];

            if let Some(items) = yaml[0].as_vec() {
                for item in items {
                    match item["add"].as_str() {
                        Some("camera") => {
                            camera = Some(item.parse()?);
                            continue;
                        }
                        Some("light") => {
                            lights.push(item.parse()?);
                            continue;
                        }
                        Some(_) => {
                            objects.push(item.parse()?);
                            continue;
                        }
                        None => (),
                    }

                    if item["define"].as_str().is_some() {
                        defines.push(item.parse()?);
                        continue;
                    }
                }
            } else {
                return Err("Expected a list of directives".to_string());
            }

            let camera = camera.ok_or("No `add: camera` directive found".to_string())?;

            Ok(SceneDescription {
                camera,
                lights,
                defines,
                objects,
            })
        }
        Err(error) => Err(error.to_string()),
    }
}
