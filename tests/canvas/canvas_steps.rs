use cucumber::{given, then, World};
use std::collections::HashMap;
use the_ray_tracer_challenge::canvas::Canvas;
use the_ray_tracer_challenge::ExtendedTuple;

#[derive(Debug, Default, World)]
pub struct CanvasWorld {
    canvases: HashMap<String, Canvas>,
}

#[given(expr = "{word} ← canvas\\({int}, {int})")]
fn canvas_is(world: &mut CanvasWorld, canvas: String, width: usize, height: usize) {
    world.canvases.insert(canvas, Canvas::new(width, height));
}

#[then(expr = "{word}.width = {int}")]
fn width_equal(world: &mut CanvasWorld, canvas: String, width: usize) {
    assert_eq!(
        world
            .canvases
            .get(&canvas)
            .expect(format!("{canvas} does not exist").as_str())
            .width(),
        &width
    );
}

#[then(expr = "{word}.height = {int}")]
fn height_equal(world: &mut CanvasWorld, canvas: String, height: usize) {
    assert_eq!(
        world
            .canvases
            .get(&canvas)
            .expect(format!("{canvas} does not exist").as_str())
            .height(),
        &height
    );
}

#[then(expr = "every pixel of {word} is color\\({float}, {float}, {float})")]
fn every_pixel_of_canvas_is_color(
    world: &mut CanvasWorld,
    canvas: String,
    red: f32,
    green: f32,
    blue: f32,
) {
    let black = ExtendedTuple::new_color(red, green, blue);
    let canvas_pixels = world
        .canvases
        .get(&canvas)
        .expect(format!("{canvas} does not exist").as_str())
        .pixels();
    canvas_pixels.iter().for_each(|pixel| {
        assert_eq!(pixel.1, &black);
    })
}
