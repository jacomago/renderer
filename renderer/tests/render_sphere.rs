use renderer::prelude::*;

#[test]
fn test_draw_small_sphere() {
    let scene = Scene::new(
        vec![SceneObject::new(
            Sphere::new(Vector3D::new(0.0, 0.0, 0.0), 1.0),
            Coloring::Fill(Color::WHITE),
        )],
        vec![],
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
.xx.
.xx.
....
"#
    );
}

#[test]
fn test_draw_sphere() {
    let l = 2;
    let dim = Dimensions::new(6 * l, 4 * l);
    let scene = Scene::new(
        vec![SceneObject::new(
            Sphere::new(Vector3D::new(0.0, 0.0, 0.0), 3.2),
            Coloring::Fill(Color::WHITE),
        )],
        vec![],
        Camera::new(
            Vector3D::new(0.0, 0.0, -20.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            10.0,
            dim,
        ),
    );
    let mut sphere_image = Image::new(dim);
    scene.render(&mut sphere_image);

    let mut circle_image = Image::new(dim);
    let circle = ColoredCircle::new(
        Circle::new(
            5.0,
            Position2D::new(dim.w() as f32 / 2.0 - 0.5, dim.h() as f32 / 2.0 - 0.5),
        ),
        Coloring::Fill(Color::WHITE),
    );

    circle.draw(&mut circle_image);
    println!("{}", sphere_image);
    println!("{}", circle_image);
    //assert_eq!(sphere_image, circle_image);
}
