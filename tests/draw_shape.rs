use renderer::prelude::*;

#[test]
fn test_draw_circle() {
    let mut image = Image::from_w_h(16, 12);
    let circle = Circle::new(5.0, Position::new(8.0, 6.0), Color::WHITE);

    circle.draw(&mut image);
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
