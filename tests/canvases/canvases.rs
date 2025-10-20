use crate::canvases_steps::CanvasWorld;
use cucumber::World;

mod canvases_steps;

#[tokio::main]
async fn main() {
    CanvasWorld::run("tests/canvases/canvases.feature").await;
}
