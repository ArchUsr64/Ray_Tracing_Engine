use super::matrix;
use super::traits::ToF64;
use std::ops;
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, right_side_value: Vec3) -> Vec3 {
        self.add_vec(&right_side_value)
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, right_side_value: Vec3) -> Vec3 {
        self.subtract_vec(&right_side_value)
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, right_side_value: Vec3) -> Vec3 {
        self.cross_product(&right_side_value)
    }
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new<T: ToF64>(x: T, y: T, z: T) -> Vec3 {
        Vec3 {
            x: (x.to_f64()),
            y: (y.to_f64()),
            z: (z.to_f64()),
        }
    }

    pub fn dot(vector_first: &Vec3, vector_second: &Vec3) -> f64 {
        vector_first.dot_product(&vector_second)
    }

    pub fn to_point(&self) -> Point3 {
        Point3::new(self.x, self.y, self.z)
    }

    pub fn dot_product(&self, other_vector: &Vec3) -> f64 {
        self.x * other_vector.x + self.y * other_vector.y + self.z * other_vector.z
    }

    pub fn cross_product(&self, other_vector: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other_vector.z - self.z * other_vector.y,
            self.z * other_vector.x - self.x * other_vector.z,
            self.x * other_vector.y - self.y * other_vector.x,
        )
    }

    pub fn normalise(&self) -> Vec3 {
        let vector_size = self.length();
        Vec3::new(
            self.x / vector_size,
            self.y / vector_size,
            self.z / vector_size,
        )
    }

    pub fn length(&self) -> f64 {
        let vector_size = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        vector_size
    }

    pub fn scale<T: ToF64 + Copy>(&self, scalar: T) -> Vec3 {
        Vec3::new(
            self.x * scalar.to_f64(),
            self.y * scalar.to_f64(),
            self.z * scalar.to_f64(),
        )
    }

    pub fn add_vec(&self, other_vector: &Vec3) -> Vec3 {
        Vec3::new(
            self.x + other_vector.x,
            self.y + other_vector.y,
            self.z + other_vector.z,
        )
    }

    pub fn subtract_vec(&self, other_vector: &Vec3) -> Vec3 {
        Vec3::new(
            self.x - other_vector.x,
            self.y - other_vector.y,
            self.z - other_vector.z,
        )
    }

    pub fn to_polar(&self) -> Vec3P {
        let theta = (self.x / self.length()).acos();
        Vec3P::new(
            self.length(),
            theta,
            (self.y / (self.length() * theta.sin())).asin(),
        )
    }
    pub fn apply_transform(&self, transformation_matrix: &matrix::Matrix3) -> Vec3 {
        Vec3::new(
            self.x * transformation_matrix.get_element(0, 0)
                + self.y * transformation_matrix.get_element(0, 1)
                + self.z * transformation_matrix.get_element(0, 2),
            self.x * transformation_matrix.get_element(1, 0)
                + self.y * transformation_matrix.get_element(1, 1)
                + self.z * transformation_matrix.get_element(1, 2),
            self.x * transformation_matrix.get_element(2, 0)
                + self.y * transformation_matrix.get_element(2, 1)
                + self.z * transformation_matrix.get_element(2, 2),
        )
    }
}

#[derive(Copy, Clone)]
pub struct Vec3P {
    pub r: f64,
    pub theta: f64,
    pub phi: f64,
}

impl Vec3P {
    pub fn new<T: ToF64>(r: T, theta: T, phi: T) -> Vec3P {
        Vec3P {
            r: r.to_f64(),
            theta: theta.to_f64(),
            phi: phi.to_f64(),
        }
    }

    pub fn to_rect(&self) -> Vec3 {
        Vec3::new(
            self.theta.sin() * self.phi.cos(),
            self.theta.sin() * self.phi.sin(),
            self.theta.cos(),
        )
        .scale(self.r)
    }
}

#[derive(Copy, Clone)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new<T: ToF64>(x: T, y: T, z: T) -> Point3 {
        Point3 {
            x: (x.to_f64()),
            y: (y.to_f64()),
            z: (z.to_f64()),
        }
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn vec_from(&self, other_point: &Point3) -> Vec3 {
        self.to_vec().subtract_vec(&other_point.to_vec())
    }
    pub fn vec_to(&self, other_point: &Point3) -> Vec3 {
        self.vec_from(&other_point).scale(-1)
    }
    pub fn distance_from(&self, other_point: &Point3) -> f64 {
        self.vec_from(&other_point).length()
    }
}
