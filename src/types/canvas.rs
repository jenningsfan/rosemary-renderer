use std::ops::{Index, IndexMut};
use super::colour::Colour;

const PPM_MAGIC: &str = "P3";
const PPM_MAX_COLOUR: f32 = 255.0;

#[derive(Debug, Clone)]
pub struct Canvas {
    canvas: Vec<Colour>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            canvas: vec![Colour::new(0.0, 0.0, 0.0); width * height],
            width,
            height,
        }
    }

    pub fn to_ppm(&self) -> String {
        let mut data = String::new();
        data += format!("{PPM_MAGIC}\n{} {}\n{}", self.width.to_string(), self.height.to_string(), PPM_MAX_COLOUR.to_string()).as_str();
        
        for (i, pixel) in self.canvas.iter().enumerate() {
            if i % self.width == 0 {
                data += "\n";
            }

            data += format!("{} {} {} ", Self::convert_colour(pixel.r),
                Self::convert_colour(pixel.g), Self::convert_colour(pixel.b)).as_str();

        }
        
        let mut new_data = String::new();
        let mut line_len = 0;
        for char in data.chars() {
            //print!("{}", char);
            line_len += 1;
            if char == ' ' && line_len >= 66 {
                new_data += " \n";
                line_len = 0;
                continue;
            }
            if char == '\n' {
                line_len = 0;
            }
            new_data += char.to_string().as_str();
        }

        new_data += "\n"; // terminator

        new_data
    }

    fn convert_colour(colour: f32) -> String {
        (((colour) * PPM_MAX_COLOUR).min(PPM_MAX_COLOUR).max(0.0).round()).to_string()
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Colour;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = index.0 + index.1 * self.width;
        &self.canvas[index]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = index.0 + index.1 * self.width;
        &mut self.canvas[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::types::colour::Colour;

    use super::Canvas;

    #[test]
    fn constructor() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        let white = Colour::new(0.0, 0.0, 0.0);
        for pixel in canvas.canvas {
            assert_eq!(pixel, white);
        }
    }

    #[test]
    fn read() {
        let mut canvas = Canvas::new(10, 20);
        let red = Colour::new(1.0, 0.0, 0.0);
        canvas.canvas[32] = red;
        assert_eq!(canvas[(2, 3)], red);
    }

    #[test]
    fn write() {
        let mut canvas = Canvas::new(10, 20);
        let red = Colour::new(1.0, 0.0, 0.0);
        canvas[(2, 3)] = red;
        assert_eq!(canvas[(2, 3)], red);
    }

    #[test]
    fn to_ppm() {
        let mut canvas = Canvas::new(5, 3);
        canvas[(0, 0)] = Colour::new(1.5, 0.0, 0.0);
        canvas[(2, 1)] = Colour::new(0.0, 0.5, 0.0);
        canvas[(4, 2)] = Colour::new(-0.5, 0.0, 1.0);

        let ppm = canvas.to_ppm();

        let header = ppm.lines().take(3).collect::<Vec<&str>>().join("\n");
        assert_eq!(header, "P3\n5 3\n255");

        let data = ppm.lines().skip(3).take(3).collect::<Vec<&str>>().join("\n");
        assert_eq!(data, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 "
        );

        let mut canvas = Canvas::new(10, 2);
        for pixel in &mut canvas.canvas {
            *pixel = Colour::new(1.0, 0.8, 0.6);
        }

        let ppm = canvas.to_ppm();

        let data = ppm.lines().skip(3).take(4).collect::<Vec<&str>>().join("\n");
        assert_eq!(data, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n\
            153 255 204 153 255 204 153 255 204 153 255 204 153 \n\
            255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 \n\
            153 255 204 153 255 204 153 255 204 153 255 204 153 "
        );

        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}