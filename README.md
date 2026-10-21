# rt: a ray tracer in Rust

`rt` renders a 3D scene to a PPM image (`P3`, ASCII). It writes the image to
standard output, so you redirect it into a file.

## How it works

For every pixel, the camera shoots several rays into the scene. Each ray finds
the closest object it hits, and the color of that point comes from two sources:

- **Direct light** from a point light, with hard shadows. A "shadow ray" is cast
  toward the lamp, and if an object blocks it, the point gets no direct light.
- **Indirect light**: the ray bounces in a random direction and asks the same
  question again, up to 8 bounces. This gives soft shadows and color bleeding
  (a red ball tints the floor next to it slightly red).

Many random rays per pixel are averaged (`--samples`), which removes the noise
and smooths the edges. Pixels are rendered in parallel on all CPU cores.

## Features

- Objects: sphere, plane, cube, cylinder (all can be placed anywhere)
- Movable camera (position and look-at point)
- Adjustable brightness and shadows
- Soft indirect lighting and a sky light
- Configurable resolution and quality from the command line
- Multithreaded rendering

## Build and run

```bash
cargo build --release
cargo run --release -- [OPTIONS] > image.ppm
cargo run --release -- --help
```

Always use `--release`. Debug builds are 10 to 50 times slower.

## Options

| Option                 | Default   | Description                                 |
| ---------------------- | --------- | ------------------------------------------- |
| `-w, --width <N>`      | 200       | Image width in pixels                       |
| `-H, --height <N>`     | 150       | Image height in pixels                      |
| `-s, --samples <N>`    | 50        | Rays per pixel (more = smoother and slower) |
| `-b, --brightness <X>` | 1.0       | Overall brightness multiplier (0 or more)   |
| `--sky <X>`            | 0.3       | Sky light strength, from 0 to 1             |
| `--scene <1-4>`        | 3         | Which scene to render (see below)           |
| `--cam-pos <x,y,z>`    | per scene | Camera position                             |
| `--look-at <x,y,z>`    | per scene | Point the camera looks at                   |

## Creating objects

Objects are added in `build_scene` in `src/bin/main.rs`. Every object takes a
`Color::new(r, g, b)` with values from 0.0 to 1.0.

```rust
// Sphere: center, radius, color
Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, Color::new(0.8, 0.1, 0.1))

// Plane: a point on it, its normal (the direction it faces), color
Plane::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Color::new(0.8, 0.8, 0.8))

// Cube: center, side length, color
Cube::new(Vec3::new(2.5, 0.0, -6.0), 2.0, Color::new(0.1, 0.3, 0.8))

// Cylinder (vertical): center of the cylinder, radius, height, color
Cylinder::new(Vec3::new(-2.5, 0.0, -6.0), 0.7, 2.0, Color::new(0.1, 0.7, 0.2))
```

Add an object to the scene with `scene.add(Box::new(object));`.

**Moving an object** means changing its center (or the plane's point). To rest
an object on a floor at `y = -1`, set its center `y` to `-1 + half its height`.

## Changing the brightness

Two things light the scene: a point light and the sky.

```bash
# half as bright
cargo run --release -- --brightness 0.5 > dim.ppm

# stronger or weaker sky only
cargo run --release -- --sky 0.1 > dusk.ppm
```

In code, the lamp is `Light::new(position, intensity)`. `--brightness`
multiplies both the lamp and the sky. Note that display gamma makes a 0.5
multiplier look closer to 70% brightness.

## Moving the camera

```bash
# camera at (-4, 2, 0), looking at (0, 0, -5)
cargo run --release -- --cam-pos=-4,2,0 --look-at=0,0,-5 > angle.ppm
```

In code: `Camera::new(position, look_at, up, width, height)`. The camera cannot
look exactly straight up or down.

## The required images

```bash
cargo run --release -- --scene 1 -w 800 -H 600 -s 200 > 1_sphere.ppm
cargo run --release -- --scene 2 -w 800 -H 600 -s 200 > 2_plane_cube_dim.ppm
cargo run --release -- --scene 3 -w 800 -H 600 -s 200 > 3_all_objects.ppm
cargo run --release -- --scene 4 -w 800 -H 600 -s 200 > 4_all_other_angle.ppm
```

| Scene | Content                                              |
| ----- | ---------------------------------------------------- |
| 1     | A sphere                                             |
| 2     | A plane and a cube, with a dimmer light than scene 1 |
| 3     | A sphere, a cube, a cylinder and a plane             |
| 4     | Scene 3 from a different camera position             |

For quick tests, use `-w 200 -H 150 -s 20`.

## Viewing the images

Open `.ppm` files with GIMP, or convert them:

```bash
convert image.ppm image.png     # ImageMagick
```
