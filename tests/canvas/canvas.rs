use crate::canvas_steps::CanvasWorld;
use cucumber::World;

mod canvas_steps;

#[tokio::main]
async fn main() {
    CanvasWorld::run("tests/canvas/canvas.feature").await;
}
