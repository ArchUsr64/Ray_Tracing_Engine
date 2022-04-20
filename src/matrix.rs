use super::traits::ToF64;
use super::traits::ToUsize;
pub struct Matrix3 {
    data: [[f64; 3]; 3],
}

pub fn matrix3_init<T: ToF64 + Copy + std::fmt::Debug>(data: &Vec<Vec<T>>) -> Matrix3 {
    if data.len() != 3 || data[0].len() != 3 {
        println!("ERROR: {:?} is not a 3x3 vector", data);
        println!("Creating an identity matrix instead");
        return identity_matrix();
    }
    Matrix3 {
        data: [
            [
                data[0][0].to_f64(),
                data[0][1].to_f64(),
                data[0][2].to_f64(),
            ],
            [
                data[1][0].to_f64(),
                data[1][1].to_f64(),
                data[1][2].to_f64(),
            ],
            [
                data[2][0].to_f64(),
                data[2][1].to_f64(),
                data[2][2].to_f64(),
            ],
        ],
    }
}

pub fn identity_matrix() -> Matrix3 {
    Matrix3 {
        data: [[1f64, 0f64, 0f64], [0f64, 1f64, 0f64], [0f64, 0f64, 1f64]],
    }
}

pub fn rotation_matrix_x<T: Copy + ToF64>(angle: T) -> Matrix3 {
    Matrix3 {
        data: [
            [1f64, 0f64, 0f64],
            [0f64, angle.to_f64().cos(), -angle.to_f64().sin()],
            [0f64, angle.to_f64().sin(), angle.to_f64().cos()],
        ],
    }
}

pub fn rotation_matrix_y<T: Copy + ToF64>(angle: T) -> Matrix3 {
    Matrix3 {
        data: [
            [angle.to_f64().cos(), 0f64, angle.to_f64().sin()],
            [0f64, 1f64, 0f64],
            [-angle.to_f64().sin(), 0f64, angle.to_f64().cos()],
        ],
    }
}

pub fn rotation_matrix_z<T: Copy + ToF64>(angle: T) -> Matrix3 {
    Matrix3 {
        data: [
            [angle.to_f64().cos(), -angle.to_f64().sin(), 0f64],
            [angle.to_f64().sin(), angle.to_f64().cos(), 0f64],
            [0f64, 0f64, 1f64],
        ],
    }
}

impl Matrix3 {
    pub fn get_element<T: ToUsize>(&self, row: T, column: T) -> f64 {
        self.data[row.to_usize()][column.to_usize()]
    }

    pub fn set_element<T: ToF64>(&mut self, row: T, column: T, new_element: T) {
        self.data[row.to_f64() as usize][column.to_f64() as usize] = new_element.to_f64();
    }

    pub fn to_vec(&self) -> Vec<Vec<f64>> {
        vec![
            self.data[0].to_vec(),
            self.data[1].to_vec(),
            self.data[2].to_vec(),
        ]
    }
}
