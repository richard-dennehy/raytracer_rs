# Describing Scenes Using YAML files

As an alternative to programmatically creating scenes using the Ray Tracer API, a YAML file can be loaded and parsed instead.

The Ray Tracer Challenge book/bonus chapters provide examples of YAML scenes, but no schema/documentation - 
this document is an attempt to reverse-engineer a description of the format.

A YAML scene must be a single document (extra documents are ignored) containing an optional list of `define` objects, and a list of `add` objects.

## Describing the Camera
All scenes must have a camera (otherwise there's no perspective to render from) and `yaml_parser::parse` will return an `Err` if the file does not contain a camera description.

A camera description must contain:
 - `width` and `height` (positive integers)
 - a `field-of-view` (in radians, i.e. `3.1415[..]` is 180 degrees)
 - `from` (the camera's position), `to` (the focal point), and `up`, as arrays of 3 floating point numbers

### Example
```yaml
- add: camera
  width: 100
  height: 100
  field-of-view: 0.785
  from: [ -6, 6, -10 ]
  to: [ 6, 0, 6 ]
  up: [ -0.45, 1, 0 ]
```

## Describing the Lights
A scene may have zero or many lights (noting that a scene with no lights will only render a black image).
The YAML parser and ray tracer support point lights, and area lights. 
Both kinds of light cast light infinitely in all directions, but a point light casts from a single point, whereas an area light
casts from an area, and therefore allows soft shadows, as a point in space can be partially lit by an area light.

A point light is a light description containing:
 - `at`, an array of 3 floating point numbers giving the light's position in 3D space
 - `intensity`, the light's colour; an array of 3 floating point numbers

### Point light example
```yaml
- add: light
  at: [ 50, 100, -50 ]
  intensity: [ 1, 1, 1 ]

# can add multiple lights
- add: light
  at: [ -400, 50, -10 ]
  intensity: [ 0.2, 0.2, 0.2 ]
```

An area light is a light description containing:
- `corner`, an array of 3 floating point numbers defining the bottom left corner of the area
- `uvec`, an array of 3 floating point numbers defining the bottom edge of the area
- `vvec`, an array of 3 floating point numbers defining the left edge of the area
- `usteps` and `vsteps`, integers defining the number of samples to take, such that the total number of samples is `usteps * vsteps`. When these values are too low, shadows will have clearly visible banding; when these values are large, rendering will take significantly longer.
- `intensity`, an array of 3 floating point numbers defining the colour of the light
### Area light example
```yaml
- add: light
  corner: [-1, 2, 4]
  uvec: [2, 0, 0]
  vvec: [0, 2, 0]
  usteps: 10
  vsteps: 10
  # there's no limit on how bright a light (or any colour) can be
  intensity: [1.5, 1.5, 1.5]
```

Note: the YAML examples from the bonus chapters also provide a `jitter` boolean value, intended to enable randomised sampling, to break up banding caused by low sample counts.
This implementation always randomly samples, so the `jitter` value has no effect and does not need to be provided.

Additionally, the rust API for area lights requires a `seed` for the RNG, to allow rendering to be deterministic. 
The YAML parser does not currently support overriding this seed, and a fixed value is used for all YAML scenes, such that rendering the same YAML file multiple times always produces the same image.

## Describing Objects
Objects are the shapes, and potentially meshes, that are ultimately rendered into an image. A scene may have zero or many objects,
noting that a scene with no objects won't be very interesting to look at.

The YAML parser supports all primitive shapes - excluding triangles - object groups, and imported meshes from Wavefront `obj` files.

An object description may contain:
 - A `material` - if this is not provided, a default grey material is used. 
   - If a material is assigned to a group, all children will use that material, even if those children define their own materials.
   - This material may reference a `define` instead of being described inline
 - A `transform` - all objects exist at the world origin, and have a fixed size and orientation. Applying transforms is the only way to affect this. 
 - `children`, if the object is a group. This must be an array of object descriptions. Groups may contain other groups.
 - `file`, if the object type is an `obj`. This file must exist within the same folder as the YAML file, and must be a Wavefront `obj` file. Note that if the `obj` file references an `mtl` library, and the `mtl` file is co-located, this file will also be loaded, and the material applied.
 - `shadow` - when `false`, excludes the object from all shadow casting calculations

### Examples
```yaml
- add: plane
  transform:
    - [ rotate-y, 0.31415 ]
  material:
    pattern:
      type: checkers
      colors:
        - [0.35, 0.35, 0.35]
        - [0.65, 0.65, 0.65]
    specular: 0
    reflective: 0.4
```

```yaml
- add: group
  transform:
    - [ translate, 0, 2, 0 ]
  children:
    - add: pedestal
    - add: group
      children:
        - add: obj
          file: dragon.obj
          material:
            color: [ 1, 0, 0.1 ]
            ambient: 0.1
            diffuse: 0.6
            specular: 0.3
            shininess: 15
        - add: cube
          material:
            ambient: 0
            diffuse: 0.4
            specular: 0
            transparency: 0.6
            refractive-index: 1
```

## Defining Common Values
To reduce repetition, or as a form of documentation, a `define` may be used to describe a material, a transform, or an object.

Defines may reference other defines, but **defines must be described before they are used**.

A define consists of a name, which can be used elsewhere in the file to reference it, and a `value`. 
Additionally, a material may `extend` another material, in which case it inherits any properties described in the parent material and not described in the child material.

### Examples
**Material**
```yaml
- define: white-material
  value:
    color: [ 1, 1, 1 ]
    diffuse: 0.7
    ambient: 0.1
    specular: 0.0
    reflective: 0.1

# Extending a material
- define: blue-material
  extend: white-material
  value:
    color: [ 0.537, 0.831, 0.914 ]
    # inherits these from `white-material`
    # diffuse: 0.7
    # ambient: 0.1
    # specular: 0.0
    # reflective: 0.1

# using defined material
- add: cube
  material: white-material
```

**Transform**
```yaml
- define: standard-transform
  value:
    - [ translate, 1, -1, 1 ]
    - [ scale, 0.5, 0.5, 0.5 ]

# applies `standard-transform`, then scales, i.e. translates by (1, -1, 1), then scales uniformly by 0.5, then again by 3.5
- define: large-object
  value:
    - standard-transform
    - [ scale, 3.5, 3.5, 3.5 ]

# using defined transform
- add: cube
  transform:
     - large-object
```

**Object**
```yaml
- define: raw-bbox
  value:
    add: cube
    shadow: false
    transform:
      - [ translate, 1, 1, 1 ]
      - [ scale, 3.73335, 2.5845, 1.6283 ]
      - [ translate, -3.9863, -0.1217, -1.1820 ]
  
- define: bbox
  value:
     add: raw-bbox
     # extra transforms are appended, rather than overriding
     transform:
        - [ translate, 0, 0.1217, 0]
        - [ scale, 0.268, 0.268, 0.268 ]  
  
# using defined object
- add: bbox
  # may override/describe material and apply extra transforms
  material:
     ambient: 0
     diffuse: 0.2
     specular: 0
     transparency: 0.8
     refractive-index: 1
```

## Describing Materials
A material object may be used as the `value` of a define, or as a property of an object. 
The `material` of an object may also be a string value referencing a `define`.

A material may contain:
 - A `color` _or_ a `pattern` - defaults to white if neither provided
   - a `color` is an array of three floating point values (RGB)
   - a `pattern` must contain:
     - `type` - one of `stripes`, `checkers`, `gradient`, `ring`, or `map` (see [UV patterns](#describing-uv-patterns))
     - `colors` - an array of 2 RGB colour values; not required by `map` patterns
     - `transform` (optional)
 - `ambient` - between 0 and 1, default 0.1
 - `diffuse` - between 0 and 1, default 0.9
 - `specular` - between 0 and 1, default 0.9
 - `shininess` - positive floating point number, default 200
 - `reflective` - between 0 and 1, default 0
 - `transparency` - between 0 and 1, default 0
 - `refractive-index` - positive floating point number, should be at least 1, default 1

See also [Phong reflection model](https://en.wikipedia.org/wiki/Phong_reflection_model)

## Describing UV patterns
An object material may use UV mapping, where 3D points on the surface of the object are converted into 2D UV coordinates,
which can then be used to project a 2D image onto a 3D object, e.g. textures.

A UV pattern can only be provided as part of a material.

This is intended to be used with single primitives - using this with e.g. triangle meshes should function, but hasn't been tested, and will likely produce strange results.

A UV pattern must contain:
- a `type` of `map`
- a `mapping`, either `planar`, `cylindrical`, `spherical`, or `cube` - see notes
- a `uv_pattern`, unless `mapping` is `cube`, which must contain:
  - a `type`, either `checkers` or `image`
  - a `width` and `height`, when `type` is `checkers` - the number of squares in the checker pattern, rows and columns, respectively
  - a pair of `colours`, when `type` is `checkers`, defining the primary and secondary colours of the checkers
  - a `file`, when the type is `image` - the name of a file, co-located with the YAML file. This must contain an image compatible with the [image crate](https://docs.rs/image/0.23.14/image/), i.e. it is not necessary to convert all images to PPM format
- optionally, `top` and `bottom` UV patterns, as described above, when `mapping` is `cylindrical`, for providing separate textures for the top and bottom caps of cylinders and cones
- `left`, `right`, `front`, `back`, `up`, and `down`, when the mapping is `cube`, UV patterns as described above, for each face of the cube

### Examples
```yaml
# within a `material`
pattern:
  type: map
  mapping: spherical
  uv_pattern:
    type: checkers
    width: 16
    height: 8
    colors:
      - [ 0, 0, 0 ]
      - [ 0.5, 0.5, 0.5 ]
```

```yaml
# within a `material`
pattern:
  type: map
  mapping: cube
  left:
    type: image
    file: negx.ppm
  right:
    type: image
    file: posx.ppm
  front:
    type: image
    file: posz.ppm
  back:
    type: image
    file: negz.ppm
  up:
    type: image
    file: posy.ppm
  down:
    type: image
    file: negy.ppm
```

Note: in this implementation, objects are responsible for providing their own UV unwrapping, and therefore `spherical` and `planar` mapping behave identically on all objects
(the motivation for creating `spherical` mapping in the bonus chapter is because UV mapping a sphere as if it were a plane results in distortion, 
but that's not possible in this implementation)

## Describing Transforms
A transform array may be used as the `value` of a define, or as a property of an object, or as a property of a `pattern` in a material.

Individual transformations in the array take effect in the order they are defined, e.g. a `translate` followed by a `scale` will move the object/pattern, then scale it _relative to the world origin_.

A transform may be one of:
 - A string value referencing a `define`
 - A `translate` - an array of four values, where the first value is `translate`, and the remaining three are the `x`, `y`, and `z` values
 - A rotation - an array of two values: a string `rotate-x`, `rotate-y`, or `rotate-z`, and a value in **radians** i.e. `3.1415[..]` will rotate 180 degrees
 - A `scale` - an array of four values, where the first value is `scale`, and the remaining three are the `x`, `y`, and `z` factors

Shear transforms are not currently supported.