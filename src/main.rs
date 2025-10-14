use std::fs;
use the_ray_tracer_challenge::{Canvas, ExtendedTuple};

fn main() {
    let position = ExtendedTuple::new_point(0f32, 1f32, 0f32);
    let velocity = ExtendedTuple::new_vector(1f32, 1.8, 0f32).normalize() * 11.25;
    let mut projectile = Projectile::new(position, velocity);

    let gravity = ExtendedTuple::new_vector(0f32, -0.1, 0f32);
    let wind = ExtendedTuple::new_vector(-0.01, 0f32, 0f32);
    let environment = Environment::new(gravity, wind);

    let mut canvas = Canvas::new(900, 550, None);
    let red = ExtendedTuple::new_color(255f32, 0f32, 0f32);
    
    while projectile.position.y() > &0f32 {
        projectile = tick(&environment, &projectile);
        canvas.write_pixel(
            *projectile.position.x() as usize,
            canvas.height() - *projectile.position.y() as usize,
            red.clone(),
        )
    }

    let ppm = canvas.convert_to_ppm();
    fs::write("./first.ppm", ppm).unwrap();
    
    println!("PPM file written");
}

struct Projectile {
    position: ExtendedTuple,
    velocity: ExtendedTuple,
}

impl Projectile {
    pub fn new(position: ExtendedTuple, velocity: ExtendedTuple) -> Self {
        if !position.is_point() {
            panic!("position must be a point");
        }
        if !velocity.is_vector() {
            panic!("velocity must be a vector");
        }
        Self { position, velocity }
    }
}

struct Environment {
    gravity: ExtendedTuple,
    wind: ExtendedTuple,
}

impl Environment {
    pub fn new(gravity: ExtendedTuple, wind: ExtendedTuple) -> Self {
        if !gravity.is_vector() {
            panic!("gravity must be a vector");
        }
        if !wind.is_vector() {
            panic!("velocity must be a vector");
        }
        Self { gravity, wind }
    }
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let position = &projectile.position + &projectile.velocity;
    let velocity = &(&projectile.velocity + &environment.gravity) + &environment.wind;
    Projectile::new(position, velocity)
}
