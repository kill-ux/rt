use rt::{camera::Camera, vec3::Vec3};

fn camera() -> Camera {
    // at the origin, looking down -z, 200x100 image
    Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        200,
        100,
    )
}

#[test]
fn center_pixel_looks_straight_ahead() {
    let ray = camera().get_ray(100.0, 50.0);
    assert!(ray.direction.x.abs() < 1e-9);
    assert!(ray.direction.y.abs() < 1e-9);
    assert!(ray.direction.z < 0.0);
}

#[test]
fn rays_start_at_the_camera_position() {
    let cam = Camera::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(1.0, 2.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        200,
        100,
    );
    let ray = cam.get_ray(10.0, 10.0);
    assert_eq!(ray.origin, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn ray_directions_are_normalized() {
    let cam = camera();
    for (x, y) in [(0.0, 0.0), (199.0, 99.0), (50.0, 70.0)] {
        assert!((cam.get_ray(x, y).direction.length() - 1.0).abs() < 1e-9);
    }
}

#[test]
fn left_pixels_point_left_and_top_pixels_point_up() {
    let cam = camera();
    let top_left = cam.get_ray(0.0, 0.0);
    let bottom_right = cam.get_ray(199.0, 99.0);
    assert!(top_left.direction.x < 0.0);
    assert!(top_left.direction.y > 0.0);
    assert!(bottom_right.direction.x > 0.0);
    assert!(bottom_right.direction.y < 0.0);
}

#[test]
fn moving_the_camera_changes_where_it_looks() {
    // camera on the right of the origin, looking at the origin: rays point left (-x)
    let cam = Camera::new(
        Vec3::new(5.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        200,
        100,
    );
    let ray = cam.get_ray(100.0, 50.0);
    assert!(ray.direction.x < -0.99);
}
