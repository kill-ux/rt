use rt::{
    color::Color,
    light::Light,
    objects::{Hittable, cube::Cube, cylinder::Cylinder, plane::Plane, sphere::Sphere},
    ray::Ray,
    scene::Scene,
    vec3::Vec3,
};

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

fn color() -> Color {
    Color::new(1.0, 1.0, 1.0)
}

// a ray from `origin` going toward `dir` (does not need to be normalized)
fn ray(origin: (f64, f64, f64), dir: (f64, f64, f64)) -> Ray {
    Ray::new(
        Vec3::new(origin.0, origin.1, origin.2),
        Vec3::new(dir.0, dir.1, dir.2).normalize(),
    )
}

// ---------- sphere ----------

fn sphere() -> Sphere {
    Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, color())
}

#[test]
fn sphere_hit_from_front() {
    let hit = sphere()
        .hit(&ray((0.0, 0.0, 0.0), (0.0, 0.0, -1.0)))
        .unwrap();
    assert!(approx(hit.t, 4.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn sphere_miss() {
    assert!(
        sphere()
            .hit(&ray((0.0, 0.0, 0.0), (0.0, 1.0, 0.0)))
            .is_none()
    );
}

#[test]
fn sphere_behind_the_ray_is_not_hit() {
    assert!(
        sphere()
            .hit(&ray((0.0, 0.0, 0.0), (0.0, 0.0, 1.0)))
            .is_none()
    );
}

#[test]
fn moved_sphere_is_hit_at_its_new_position() {
    let s = Sphere::new(Vec3::new(3.0, 0.0, -5.0), 1.0, color());
    assert!(s.hit(&ray((0.0, 0.0, 0.0), (0.0, 0.0, -1.0))).is_none());
    assert!(s.hit(&ray((3.0, 0.0, 0.0), (0.0, 0.0, -1.0))).is_some());
}

// ---------- plane ----------

fn floor() -> Plane {
    Plane::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), color())
}

#[test]
fn plane_hit_from_above() {
    let hit = floor()
        .hit(&ray((0.0, 0.0, 0.0), (0.0, -1.0, 0.0)))
        .unwrap();
    assert!(approx(hit.t, 1.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
}

#[test]
fn plane_hit_from_below_flips_the_normal() {
    let hit = floor()
        .hit(&ray((0.0, -3.0, 0.0), (0.0, 1.0, 0.0)))
        .unwrap();
    assert!(approx(hit.t, 2.0));
    assert_eq!(hit.normal, Vec3::new(0.0, -1.0, 0.0));
}

#[test]
fn plane_parallel_ray_misses() {
    assert!(
        floor()
            .hit(&ray((0.0, 0.0, 0.0), (1.0, 0.0, 0.0)))
            .is_none()
    );
}

#[test]
fn plane_behind_the_ray_is_not_hit() {
    assert!(
        floor()
            .hit(&ray((0.0, 0.0, 0.0), (0.0, 1.0, 0.0)))
            .is_none()
    );
}

// ---------- cube ----------

fn cube() -> Cube {
    Cube::new(Vec3::new(0.0, 0.0, -5.0), 2.0, color())
}

#[test]
fn cube_hit_front_face() {
    let hit = cube().hit(&ray((0.0, 0.0, 0.0), (0.0, 0.0, -1.0))).unwrap();
    assert!(approx(hit.t, 4.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn cube_hit_side_face() {
    let hit = cube()
        .hit(&ray((-5.0, 0.0, -5.0), (1.0, 0.0, 0.0)))
        .unwrap();
    assert!(approx(hit.t, 4.0));
    assert_eq!(hit.normal, Vec3::new(-1.0, 0.0, 0.0));
}

#[test]
fn cube_miss() {
    assert!(
        cube()
            .hit(&ray((0.0, 3.0, 0.0), (0.0, 0.0, -1.0)))
            .is_none()
    );
}

#[test]
fn cube_hit_from_inside_returns_the_wall() {
    let hit = cube()
        .hit(&ray((0.0, 0.0, -5.0), (0.0, 0.0, -1.0)))
        .unwrap();
    assert!(approx(hit.t, 1.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0)); // outward normal
}

// ---------- cylinder (center is the middle of the cylinder) ----------

fn cylinder() -> Cylinder {
    Cylinder::new(Vec3::new(0.0, 0.0, -5.0), 1.0, 2.0, color())
}

#[test]
fn cylinder_hit_on_the_side() {
    let hit = cylinder()
        .hit(&ray((0.0, 0.0, 0.0), (0.0, 0.0, -1.0)))
        .unwrap();
    assert!(approx(hit.t, 4.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn cylinder_hit_on_the_top_cap() {
    // a vertical ray (parallel to the axis) from above
    let hit = cylinder()
        .hit(&ray((0.0, 5.0, -5.0), (0.0, -1.0, 0.0)))
        .unwrap();
    assert!(approx(hit.t, 4.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
}

#[test]
fn cylinder_ray_above_the_top_misses() {
    assert!(
        cylinder()
            .hit(&ray((0.0, 3.0, 0.0), (0.0, 0.0, -1.0)))
            .is_none()
    );
}

#[test]
fn cylinder_vertical_ray_outside_the_circle_misses() {
    assert!(
        cylinder()
            .hit(&ray((3.0, 5.0, -5.0), (0.0, -1.0, 0.0)))
            .is_none()
    );
}

// ---------- scene: closest hit and shadows ----------

#[test]
fn closest_hit_picks_the_nearer_object() {
    let mut scene = Scene::new(Light::new(Vec3::new(0.0, 10.0, 0.0), 1.0));
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -10.0),
        1.0,
        color(),
    ))); // far
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -5.0),
        1.0,
        color(),
    ))); // near

    let hit = scene
        .closest_hit(&ray((0.0, 0.0, 0.0), (0.0, 0.0, -1.0)))
        .unwrap();
    assert!(approx(hit.t, 4.0));
}

#[test]
fn point_under_an_object_is_in_shadow() {
    let mut scene = Scene::new(Light::new(Vec3::new(0.0, 5.0, 0.0), 1.0));
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        1.0,
        color(),
    )));

    assert!(scene.is_shadowed(Vec3::new(0.0, 0.0, 0.0))); // blocked by the sphere
    assert!(!scene.is_shadowed(Vec3::new(5.0, 0.0, 0.0))); // clear line to the lamp
}

#[test]
fn an_object_behind_the_light_casts_no_shadow() {
    let mut scene = Scene::new(Light::new(Vec3::new(0.0, 3.0, 0.0), 1.0));
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 8.0, 0.0),
        1.0,
        color(),
    )));

    assert!(!scene.is_shadowed(Vec3::new(0.0, 0.0, 0.0)));
}
