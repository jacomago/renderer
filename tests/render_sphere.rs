use renderer::prelude::*;

#[test]
fn test_draw_small_sphere() {
    let scene = Scene::new(
        vec![Box::new(Sphere::new(
            Vector3D::new(0.0, 0.0, 0.0),
            1.0,
            Coloring::Fill(Color::WHITE),
        ))],
        Camera::new(
            Vector3D::new(0.0, 0.0, -4.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            2.0,
            Dimensions::new(4, 4),
        ),
    );
    let image = scene.render();

    assert_eq!(
        format!("{}", image),
        r#"....
.xx.
.xx.
....
"#
    );
}

#[test]
fn test_draw_sphere() {
    let scene = Scene::new(
        vec![Box::new(Sphere::new(
            Vector3D::new(0.0, 0.0, 0.0),
            5.0,
            Coloring::Fill(Color::WHITE),
        ))],
        Camera::new(
            Vector3D::new(0.0, 0.0, -20.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            10.0,
            Dimensions::new(16, 12),
        ),
    );
    let image = scene.render();

    assert_eq!(
        format!("{}", image),
        r#"................
................
......xxxxx.....
.....xxxxxxx....
....xxxxxxxxx...
....xxxxxxxxx...
....xxxxxxxxx...
....xxxxxxxxx...
....xxxxxxxxx...
.....xxxxxxx....
......xxxxx.....
................
"#
    );
}
