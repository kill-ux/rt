# rt
ray tracing


```
rt/
├── Cargo.toml
├── README.md                  # Your documentation (required by subject)
├── images/                    # The 4 required .ppm output images
│   ├── sphere.ppm
│   ├── plane_cube.ppm
│   ├── all_objects.ppm
│   └── all_objects_cam2.ppm
│
└── src/
    ├── main.rs                # Entry point, CLI args, scene setup, PPM output
    │
    ├── vec3.rs                # Vec3 struct: add, sub, dot, cross, normalize, etc.
    ├── ray.rs                 # Ray struct: origin + direction, point_at(t)
    ├── camera.rs              # Camera: position, angle, generate primary rays
    │
    ├── objects/
    │   ├── mod.rs             # Hittable trait (hit, normal, color)
    │   ├── sphere.rs          # Sphere intersection math
    │   ├── plane.rs           # Flat plane intersection math
    │   ├── cube.rs            # Cube (AABB) intersection math
    │   └── cylinder.rs        # Cylinder intersection math
    │
    ├── scene.rs               # Scene struct: list of objects + lights
    ├── light.rs               # Light struct: position, brightness/intensity
    ├── color.rs               # Color struct, clamping to 0-255, RGB output
    ├── renderer.rs            # Core ray tracing loop, shadow rays, shading
    │
    └── tests/
        ├── vec3_tests.rs      # Unit tests: dot, cross, normalize
        ├── intersection_tests.rs  # Unit tests: ray vs sphere/cube/plane/cylinder
        ├── color_tests.rs     # Unit tests: brightness scaling, RGB clamping
        └── camera_tests.rs    # Unit tests: camera transforms, ray directions
```