use super::matrix::matrix3_init;

use super::traits::ToF64;

use super::vector::Point3;
use super::vector::Vec3;
use super::vector::Vec3P;

use std::f64::consts::PI;

pub struct Camera {
    pub position: Point3,
    pub yaw: f64,
    pub pitch: f64,
    pub fov: f64,
}


impl Camera {

    pub fn new<T: ToF64 + Copy>(position: &Point3, yaw: T, pitch: T, fov: T) -> Camera {
        Camera {
            position: Point3::new(position.x, position.y, position.z),
            yaw: yaw.to_f64(),
            pitch: pitch.to_f64(),
            fov: fov.to_f64(),
        }
    }

    pub fn view_vec(&self) -> Vec3 {
        let view_vector_polar = Vec3P::new(1f64, PI / 2f64 - self.pitch, self.yaw);
        let view_vector = view_vector_polar.to_rect();
        let transformation_matrix = matrix3_init(&vec![
            vec![1f64, 0f64, 0f64],
            vec![0f64, 0f64, 1f64],
            vec![0f64, 1f64, 0f64],
        ]);
        view_vector.apply_transform(&transformation_matrix)
    }
}
