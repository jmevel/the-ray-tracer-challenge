mod matrices_steps;

use cucumber::World;
pub use matrices_steps::MatrixWorld;

#[tokio::main]
async fn main() {
    MatrixWorld::run("tests/matrices/matrices.feature").await;
}
