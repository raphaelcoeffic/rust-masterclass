trait Drawable {
    fn draw(&self);
}

trait Resizable {
    fn resize(&mut self, factor: f64);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Resizable for Circle {
    fn resize(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

// Generic function with trait bounds
fn render<T: Drawable>(shape: &T) {
    shape.draw();
}

// Multiple trait bounds
fn resize_and_render<T: Drawable + Resizable>(shape: &mut T, factor: f64) {
    shape.resize(factor);
    shape.draw();
}

// Alternative syntax
fn resize_and_render_alt<T>(shape: &mut T, factor: f64)
where
    T: Drawable + Resizable,
{
    shape.resize(factor);
    shape.draw();
}

fn main() {
    let mut circle = Circle { radius: 1.0 };

    render(&mut circle);
    resize_and_render(&mut circle, 2.0);
    resize_and_render_alt(&mut circle, 2.0);
}
