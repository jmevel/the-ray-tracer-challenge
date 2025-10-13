use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use std::collections::HashMap;
use the_ray_tracer_challenge::canvas::Canvas;
use the_ray_tracer_challenge::ExtendedTuple;

#[derive(Debug, Default, World)]
pub struct CanvasWorld {
    canvases: HashMap<String, Canvas>,
    tuples: HashMap<String, ExtendedTuple>,
    ppms: HashMap<String, String>,
}

#[given(expr = "{word} ← canvas\\({int}, {int})")]
fn canvas_is(world: &mut CanvasWorld, canvas: String, width: usize, height: usize) {
    world
        .canvases
        .insert(canvas, Canvas::new(width, height, None));
}

#[given(expr = "{word} ← color\\({float}, {float}, {float})")]
fn tuple_is_color(world: &mut CanvasWorld, tuple: String, red: f32, green: f32, blue: f32) {
    world
        .tuples
        .insert(tuple, ExtendedTuple::new_color(red, green, blue));
}

#[when(expr = "write_pixel\\({word}, {int}, {int}, {word})")]
fn write_pixels(world: &mut CanvasWorld, canvas: String, x: usize, y: usize, color: String) {
    let color = world.get_tuple(color).to_owned();
    world.get_mut_canvas(canvas).write_pixel(x, y, color)
}

#[when(expr = "{word} ← canvas_to_ppm\\({word})")]
fn convert_to_ppm(world: &mut CanvasWorld, ppm: String, canvas: String) {
    let canvas = world.get_canvas(canvas);
    let result = canvas.convert_to_ppm();
    world.ppms.insert(ppm, result);
}

#[when(expr = "every pixel of {word} is set to color\\({float}, {float}, {float})")]
fn every_pixel_of_canvas_is_set_to_color(
    world: &mut CanvasWorld,
    canvas_name: String,
    red: f32,
    green: f32,
    blue: f32,
) {
    let color = ExtendedTuple::new_color(red, green, blue);
    let canvas = world.get_mut_canvas(canvas_name);
    canvas.write_all_pixels(color);
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
fn pixel_at(world: &mut CanvasWorld, canvas: String, x: usize, y: usize, color: String) {
    let expected = world.get_tuple(color);
    assert_eq!(
        world.get_canvas(canvas).pixels().get(&(x, y)).unwrap(),
        expected
    );
}

#[then(expr = "lines {int}-{int} of {word} are")]
fn lines_of_ppm_are(
    world: &mut CanvasWorld,
    first_line: usize,
    last_line: usize,
    ppm: String,
    step: &Step,
) {
    let lines_range = first_line - 1..last_line;
    let lines = world
        .get_ppm(ppm)
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if lines_range.contains(&i) {
                Some(line)
            } else {
                None
            }
        })
        .collect();
    let lines: Vec<&str> = lines;
    step.docstring
        .as_ref()
        .unwrap()
        .lines()
        .skip(1)
        .enumerate()
        .for_each(|(i, line)| {
            assert_eq!(line.to_string(), lines[i].to_string());
        });
}

#[then(expr = "{word} ends with a newline character")]
fn ppm_ends_with_a_new_line_character(world: &mut CanvasWorld, ppm: String) {
    assert_eq!(world.get_ppm(ppm).chars().last().unwrap(), '\n');
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
    fn get_ppm(&self, ppm: String) -> &str {
        self.ppms
            .get(&ppm)
            .expect(format!("{ppm} does not exist").as_str())
    }
}
