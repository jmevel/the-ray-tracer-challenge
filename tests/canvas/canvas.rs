use cucumber::World;
use crate::canvas_steps::CanvasWorld;

mod canvas_steps;

#[tokio::main]
async fn main() {
    CanvasWorld::run("tests/canvas/canvas.feature").await;
}
