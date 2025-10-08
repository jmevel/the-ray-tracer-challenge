use the_ray_tracer_challenge::vector::Vector;
use the_ray_tracer_challenge::ExtendedTuple;
use the_ray_tracer_challenge::point::Point;

fn main() {
    let mut projectile = Projectile::new(
        Point::new(0f32, 1f32, 0f32, None),
        Vector::new(1f32, 1f32, 0f32, None).normalize(),
    );
    let environment = Environment::new(
        Vector::new(0f32, -0.1, 0f32, None),
        Vector::new(-0.01, 0f32, 0f32, None),
    );

    println!("initial position: {:#?}", projectile.position);

    let mut tick_count = 0;
    while projectile.position.y() > &0f32 {
        projectile = tick(&environment, &projectile);
        tick_count += 1;
        println!("tick {}, position: {:#?}", tick_count, projectile.position);
    }
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
