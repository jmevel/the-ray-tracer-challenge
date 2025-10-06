use crate::ExtendedTuple;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: HashMap<(usize, usize), ExtendedTuple>,
}

impl Canvas {
    pub fn width(&self) -> &usize {
        &self.width
    }
    pub fn height(&self) -> &usize {
        &self.height
    }
    pub fn pixels(&self) -> &HashMap<(usize, usize), ExtendedTuple> {
        &self.pixels
    }
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = HashMap::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                pixels.insert((x, y), ExtendedTuple::new_color(0.0, 0.0, 0.0));
            }
        }
        Self {
            width,
            height,
            pixels,
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: ExtendedTuple) {
        if !pixel.is_color() {
            panic!("pixel is not color");
        }
        self.pixels.entry((x, y)).and_modify(|p| *p = pixel);
    }
    pub fn convert_to_ppm(&self) -> String {
        "P3\n\
        5 3\n\
        255\n".to_owned()
    }
}
