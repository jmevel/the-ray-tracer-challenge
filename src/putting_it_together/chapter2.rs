use std::fs;
use the_ray_tracer_challenge::{Canvas, Color, Point, Tuple, Vector};

#[allow(dead_code)]
pub fn putting_it_together() {
    let position = Point::new_point(0.0, 1.0, 0.0);
    let velocity = Vector::new_vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut projectile = Projectile::new(position, velocity);

    let gravity = Vector::new_vector(0.0, -0.1, 0.0);
    let wind = Vector::new_vector(-0.01, 0.0, 0.0);
    let environment = Environment::new(gravity, wind);

    let mut canvas = Canvas::new(900, 550, None);

    while projectile.position.y() > 0.0 {
        projectile = tick(&environment, &projectile);
        canvas.write_pixel(
            projectile.position.x() as usize,
            canvas.height() - projectile.position.y() as usize,
            Color::red(),
        )
    }

    let ppm = canvas.convert_to_ppm();
    fs::write("./resources/images/chapter 2/chapter2.ppm", ppm).unwrap();

    println!("PPM file written");
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let position = &projectile.position + &projectile.velocity;
    let velocity = &(&projectile.velocity + &environment.gravity) + &environment.wind;
    Projectile::new(position, velocity)
}
