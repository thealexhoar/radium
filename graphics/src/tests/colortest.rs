use Color;

#[test]
fn test_f_construction() {
    let color1 = Color::new_from_rgb_f(1.,0.,0.5);
    let color2 = Color::new_from_rgba_f(0.2, 0.4, 0.6, 0.8);

    assert_eq!(color1.r, 255);
    assert_eq!(color1.g, 0);
    assert_eq!(color1.b, 127);
    assert_eq!(color1.a, 255);
    assert_eq!(color2.r, 51);
    assert_eq!(color2.g, 102);
    assert_eq!(color2.b, 153);
    assert_eq!(color2.a, 204);
}

#[test]
fn test_eq() {
    let color1 = Color::red();
    let color2 = Color::new_from_rgb(255,0,0);

    assert_eq!(color1, color2);
}