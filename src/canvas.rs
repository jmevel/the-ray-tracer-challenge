use crate::Color;
use crate::tuple::Tuple;
use std::cmp;
use std::collections::HashMap;

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
                pixels.insert((x, y), Color::new_color(0.0, 0.0, 0.0));
            }
        }
        Self {
            width,
            height,
            pixels,
            max_color_value,
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels.entry((x, y)).and_modify(|p| *p = color);
    }
    pub fn write_all_pixels(&mut self, color: Color) {
        self.pixels.values_mut().for_each(|p| *p = color.clone());
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
            let mut line = "".to_owned();
            for x in 0..self.width {
                let separator = if x != 0 { " " } else { "" };
                let pixel = self.pixels.get(&(x, y)).unwrap();
                let pixel_to_ppm = format!(
                    "{}{} {} {}",
                    separator,
                    self.scale_color(pixel.x()),
                    self.scale_color(pixel.y()),
                    self.scale_color(pixel.z())
                );
                line.push_str(pixel_to_ppm.as_str());
            }
            // each line must not be more than 70 characters long
            if line.chars().count() > 70 {
                // we can't split a line in the middle of a number, it must be a space
                let split_index = find_index_to_split_at(line.as_str(), 70);
                let (first, second) = line.split_at(split_index.unwrap());
                ppm.push_str(first);
                ppm.push('\n');
                ppm.push_str(second.trim());
            } else {
                ppm.push_str(line.as_str());
            }

            ppm.push('\n');
        }

        // must end with a new line character
        ppm.push('\n');

        ppm
    }

    fn scale_color(&self, color: &f32) -> u32 {
        cmp::min((color * self.max_color_value as f32).round() as u32, 255)
    }
}

fn find_index_to_split_at(line: &str, index: usize) -> Option<usize> {
    if line.chars().nth(index) == Some(' ') {
        return Some(index);
    }
    let mut new_index = index;
    new_index -= 1;
    find_index_to_split_at(line, new_index)
}
