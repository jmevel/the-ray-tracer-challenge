mod transformations_steps;

use cucumber::World;
pub use transformations_steps::TransformationWorld;

#[tokio::main]
async fn main() {
    TransformationWorld::run("tests/transformations/transformations.feature").await;
}
