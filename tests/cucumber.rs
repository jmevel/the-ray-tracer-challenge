mod steps;

use cucumber::World;
use steps::ray_tracer_world::RayTracerWorld;

#[tokio::main]
async fn main() {
    RayTracerWorld::run("tests/features").await;
}
