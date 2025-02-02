use specs::prelude::*;
use cgmath::{Vector2, Vector3, Zero};

type Pixel = Vector3<f64>;

#[derive(Component)]
pub struct Image {
    pub resolution: Vector2<u32>,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(resolution: Vector2<u32>) -> Self {
        let pixels = (0..resolution.x * resolution.y)
            .map(|_| Pixel::zero())
            .collect();

        Self { resolution, pixels }
    }

    pub fn get(&self, coordinate: &Vector2<u32>) -> &Pixel {
        let offset = self.offset(coordinate);
        &self.pixels[offset]
    }

    pub fn set(&mut self, coordinate: &Vector2<u32>, pixel: Pixel) {
        let offset = self.offset(coordinate);
        self.pixels[offset] = pixel;
    }

    fn offset(&self, coordinate: &Vector2<u32>) -> usize {
        (self.resolution.x * coordinate.y + coordinate.x) as usize
    }
}

#[cfg(test)]
mod test;