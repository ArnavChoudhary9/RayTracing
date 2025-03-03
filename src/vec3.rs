use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub e: [f64; 3],
}

impl Vector3 {
    // Constructors
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { e: [x, y, z] }
    }

    pub fn default() -> Self {
        Vector3 { e: [0.0, 0.0, 0.0] }
    }

    // Components
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    // Operaations
    pub fn lenght(&self) -> f64 { self.squared_length().sqrt() }

    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn normalized(&self) -> Vector3 {
        let length = self.lenght();
        *self / length
    }
}

// Math operators
impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Self) -> Self::Output {
        Vector3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Self::Output {
        Vector3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector3::new(
            self.e[0] * scalar,
            self.e[1] * scalar,
            self.e[2] * scalar,
        )
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        Vector3::new(
            self * other.e[0],
            self * other.e[1],
            self * other.e[2],
        )
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Self::Output {
        Vector3::new(
            self.e[0] / scalar,
            self.e[1] / scalar,
            self.e[2] / scalar,
        )
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

// Indexing
impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}
