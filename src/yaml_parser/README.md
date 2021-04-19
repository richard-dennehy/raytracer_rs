# Describing Scenes Using YAML files

As an alternative to programmatically creating scenes using the Ray Tracer API, a YAML file can be loaded and parsed instead.

The Ray Tracer Challenge book/bonus chapters provide examples of YAML scenes, but no schema/documentation - 
this document is an attempt to reverse-engineer a description of the format.

A YAML scene must be a single document (extra documents are ignored) containing an optional list of `define` objects, and a list of `add` objects.

## Describing the Camera
All scenes must have a camera (otherwise there's no perspective to render from) and `yaml_parser::parse` will return an `Err` if the file does not contain a camera description.

A camera description must contain:
 - `width` and `height` (positive integers)
 - a `field-of-view` (in radians, i.e. `2.0` is 360 degrees)
 - `from` (the camera's position), `to` (the focal point), and `up`, as arrays of 3 floating point numbers

#### Example
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
The YAML parser (and ray tracer generally) currently only supports point lights - 
infinitely bright single points in space, which cast light rays in all directions equally, and create sharp shadows.

A light description must contain:
 - `at`, an array of 3 floating point numbers giving the light's position in 3D space
 - `intensity`, the light's colour; an array of 3 floating point numbers

#### Example
```yaml
- add: light
  at: [ 50, 100, -50 ]
  intensity: [ 1, 1, 1 ]

# can add multiple lights
- add: light
  at: [ -400, 50, -10 ]
  intensity: [ 0.2, 0.2, 0.2 ]
```

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
 - `file`, if the object type is an `obj`. This file must exist within `scene_descriptions/obj_files`, and must be a Wavefront `obj` file.

#### Example

## Defining Common Values