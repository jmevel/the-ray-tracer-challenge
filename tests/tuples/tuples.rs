mod tuples_steps;

use cucumber::World;
pub use tuples_steps::TupleWorld;

#[tokio::main]
async fn main() {
    TupleWorld::run("tests/tuples/tuples.feature").await;
}
