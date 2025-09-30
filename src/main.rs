use the_ray_tracer_challenge::ExtendedTuple;

fn main() {
    let mut projectile = Projectile::new(
        ExtendedTuple::new_point(0f32, 1f32, 0f32),
        ExtendedTuple::new_vector(1f32, 1f32, 0f32).normalize(),
    );
    let environment = Environment::new(
        ExtendedTuple::new_vector(0f32, -0.1, 0f32),
        ExtendedTuple::new_vector(-0.01, 0f32, 0f32),
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
