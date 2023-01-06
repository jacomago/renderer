use renderer::prelude::*;

fn main() {
    let mut image = Image::from_w_h(64, 48);
    let circle = Circle::new(5.0, Position::new(32.0, 24.0), Color::WHITE);
    
    circle.draw(&mut image);
    println!("{}", image);
}
