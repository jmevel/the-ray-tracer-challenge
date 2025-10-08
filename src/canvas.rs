use std::cmp;
use std::collections::HashMap;
use crate::color::Color;

#[derive(Debug, Default)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: HashMap<(usize, usize), Color>,
    max_color_value: u32,
}

impl Canvas {
    pub fn width(&self) -> &usize {
        &self.width
    }
    pub fn height(&self) -> &usize {
        &self.height
    }
    pub fn pixels(&self) -> &HashMap<(usize, usize), Color> {
        &self.pixels
    }
    pub fn new(width: usize, height: usize, max_color_value: Option<u32>) -> Self {
        let max_color_value = max_color_value.unwrap_or(255);
        let mut pixels = HashMap::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                pixels.insert((x, y), Color::new(0.0, 0.0, 0.0));
            }
        }
        Self {
            width,
            height,
            pixels,
            max_color_value,
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Color) {
        self.pixels.entry((x, y)).and_modify(|p| *p = pixel);
    }
    pub fn convert_to_ppm(&self) -> String {
        let header = format!(
            "P3\n\
        {} {}\n\
        {}\n",
            self.width, self.height, self.max_color_value
        );

        let mut ppm = header;
        for y in 0..self.height {
            for x in 0..self.width {
                let separator = if x != 0 { " " } else { "" };
                let pixel = self.pixels.get(&(x, y)).unwrap();
                let pixel_to_ppm = format!(
                    "{}{} {} {}",
                    separator,
                    self.scale_color(pixel.red()),
                    self.scale_color(pixel.green()),
                    self.scale_color(pixel.blue())
                );
                ppm.push_str(pixel_to_ppm.as_str());
            }
            ppm.push('\n');
        }

        ppm
    }

    fn scale_color(&self, color: &f32) -> u32 {
        cmp::min((color * self.max_color_value as f32).round() as u32, 255)
    }
}
