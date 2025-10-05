use cucumber::{World, given, then, when};
use std::collections::HashMap;
use the_ray_tracer_challenge::ExtendedTuple;
use the_ray_tracer_challenge::canvas::Canvas;

#[derive(Debug, Default, World)]
pub struct CanvasWorld {
    canvases: HashMap<String, Canvas>,
    tuples: HashMap<String, ExtendedTuple>,
}

#[given(expr = "{word} ← canvas\\({int}, {int})")]
fn canvas_is(world: &mut CanvasWorld, canvas: String, width: usize, height: usize) {
    world.canvases.insert(canvas, Canvas::new(width, height));
}

#[given(expr = "{word} ← color\\({float}, {float}, {float})")]
fn tuple_is_color(world: &mut CanvasWorld, tuple: String, x: f32, y: f32, z: f32) {
    world
        .tuples
        .insert(tuple, ExtendedTuple::new_point(x, y, z));
}

#[when(expr = "write_pixel\\({word}, {int}, {int}, {word})")]
fn write_pixels(world: &mut CanvasWorld, canvas: String, x: usize, y: usize, color: String) {
    let test: ExtendedTuple = world.get_tuple(color).to_owned();
    world.get_mut_canvas(canvas).write_pixel(x, y, test)
}

#[then(expr = "{word}.width = {int}")]
fn width_equal(world: &mut CanvasWorld, canvas: String, width: usize) {
    assert_eq!(world.get_canvas(canvas).width(), &width);
}

#[then(expr = "{word}.height = {int}")]
fn height_equal(world: &mut CanvasWorld, canvas: String, height: usize) {
    assert_eq!(world.get_canvas(canvas).height(), &height);
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
    let canvas_pixels = world.get_canvas(canvas).pixels();
    canvas_pixels.iter().for_each(|pixel| {
        assert_eq!(pixel.1, &black);
    })
}

#[then(expr = "pixel_at\\({word}, {int}, {int}) = {word}")]
fn pixel_at(
    world: &mut CanvasWorld,
    canvas: String,
    x: usize,
    y: usize,
    color: String,
) {
    let expected = world.get_tuple(color);
    assert_eq!(world.get_canvas(canvas).pixels().get(&(x, y)).unwrap(),expected);
}

impl CanvasWorld {
    fn get_canvas(&self, canvas: String) -> &Canvas {
        self.canvases
            .get(&canvas)
            .expect(format!("{canvas} does not exist").as_str())
    }
    fn get_mut_canvas(&mut self, canvas: String) -> &mut Canvas {
        self.canvases
            .get_mut(&canvas)
            .expect(format!("{canvas} does not exist").as_str())
    }
    fn get_tuple(&self, tuple: String) -> &ExtendedTuple {
        self.tuples
            .get(&tuple)
            .expect(format!("{tuple} does not exist").as_str())
    }
}
