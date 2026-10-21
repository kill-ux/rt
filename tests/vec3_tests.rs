use rt::vec3::Vec3;

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn dot_of_perpendicular_vectors_is_zero() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    assert!(approx(a.dot(&b), 0.0));
}

#[test]
fn dot_of_parallel_vectors() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    assert!(approx(a.dot(&a), 14.0));
}

#[test]
fn length_of_3_4_0_is_5() {
    assert!(approx(Vec3::new(3.0, 4.0, 0.0).length(), 5.0));
}

#[test]
fn normalize_gives_length_one() {
    let n = Vec3::new(3.0, 4.0, 12.0).normalize();
    assert!(approx(n.length(), 1.0));
}

#[test]
fn cross_of_x_and_y_is_z() {
    let c = Vec3::new(1.0, 0.0, 0.0).cross(&Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(c, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn cross_is_perpendicular_to_both_inputs() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(-2.0, 0.5, 4.0);
    let c = a.cross(&b);
    assert!(approx(c.dot(&a), 0.0));
    assert!(approx(c.dot(&b), 0.0));
}

#[test]
fn add_sub_and_scale() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
    assert_eq!(b - a, Vec3::new(3.0, 3.0, 3.0));
    assert_eq!(a * 2.0, Vec3::new(2.0, 4.0, 6.0));
}

#[test]
fn random_unit_vectors_have_length_one() {
    for _ in 0..1000 {
        assert!(approx(Vec3::random_unit().length(), 1.0));
    }
}
