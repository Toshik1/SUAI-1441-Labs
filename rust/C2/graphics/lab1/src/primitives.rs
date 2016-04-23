use sdl2::pixels::Color;
use sdl2;
use matrix::*;

pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

pub trait Primitive2D {
    fn to_matrix(&self) -> Matrix;
    fn from_matrix(&mut self, m: &Matrix);

    fn draw<F: FnMut(i32, i32, Color)>
        (&self, set_pixel: F);

    fn translate(&mut self, dx: f32, dy: f32) {
        let obj = self.to_matrix();
        let (x, y) = (obj.matrix[2][0], obj.matrix[2][1]);

        self.from_matrix(
          &(obj *
            translation_matrix(-x, -y) *
            translation_matrix(dx, dy) *
            translation_matrix(x, y))
        );
    }

    fn scale(&mut self, sx: f32, sy: f32) {
        let obj = self.to_matrix();
        let (x, y) = (obj.matrix[2][0], obj.matrix[2][1]);

        self.from_matrix(
          &(obj *
            translation_matrix(-x, -y) *
            scale_matrix(sx, sy) *
            translation_matrix(x, y))
        );
    }

    fn rotate(&mut self, angle: f32) {
        let obj = self.to_matrix();
        let (x, y) = (obj.matrix[2][0], obj.matrix[2][1]);

        self.from_matrix(
          &(obj *
            translation_matrix(-x, -y) *
            rotation_matrix(angle) *
            translation_matrix(x, y))
        );
    }
}
