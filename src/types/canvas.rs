use std::ops::{Index, IndexMut};
use super::colour::Colour;

#[derive(Debug, Clone)]
struct Canvas {
    canvas: Vec<Colour>,
    width: usize,
    height: usize,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Self {
            canvas: vec![Colour::new(0.0, 0.0, 0.0); width * height],
            width,
            height,
        }
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
}