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
            Vector3D::new(0.0, 0.0, -6.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            2.0,
            Dimensions::new(4, 4),
        ),
    );
    let mut image = Image::new(Dimensions::new(4, 4));
    scene.render(&mut image);

    assert_eq!(
        format!("{}", image),
        r#"....
..x.
.xxx
..x.
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
            Vector3D::new(0.0, 0.0, -60.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            5.0,
            Dimensions::new(16, 12),
        ),
    );
    let mut image = Image::new(Dimensions::new(16, 12));
    scene.render(&mut image);

    assert_eq!(
        format!("{}", image),
        r#"................
......xxxxx.....
.....xxxxxxx....
....xxxxxxxxx...
...xxxxxxxxxxx..
...xxxxxxxxxxx..
...xxxxxxxxxxx..
...xxxxxxxxxxx..
...xxxxxxxxxxx..
....xxxxxxxxx...
.....xxxxxxx....
......xxxxx.....
"#
    );
}
