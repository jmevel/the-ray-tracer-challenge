mod steps;

use cucumber::World;
use steps::ray_tracer_world::RayTracerWorld;

#[tokio::main]
async fn main() {
    RayTracerWorld::cucumber()
        .fail_fast()
        .fail_on_skipped()
        .run_and_exit("tests/features")
        .await;
}
