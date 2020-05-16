use crate::windows::foundation::numerics::{Vector2, Vector3};
use std::ops::{Add, Div, Mul, Sub};

impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Vector2> for Vector2 {
    type Output = Self;
    fn add(self, other: &Vector2) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Vector2 {
    type Output = Vector2;
    fn add(self, other: Self) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<&Vector2> for Vector2 {
    type Output = Self;
    fn sub(self, other: &Vector2) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub for &Vector2 {
    type Output = Vector2;
    fn sub(self, other: Self) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Div for Vector2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self * (1.0 / rhs)
    }
}

impl Div<f32> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        self * (1.0 / rhs)
    }
}

impl Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<f32> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub trait FromVector2 {
    type Output;
    fn from_vector2(value: Vector2, z: f32) -> Self::Output;
}

impl FromVector2 for Vector3 {
    type Output = Self;
    fn from_vector2(value: Vector2, z: f32) -> Self {
        Vector3 {
            x: value.x,
            y: value.y,
            z,
        }
    }
}

#[test]
fn vector2_add() {
    let value1 = Vector2 { x: 5.0, y: 50.0 };
    let value2 = Vector2 { x: 15.0, y: 25.0 };

    let result = value1.clone() + value2.clone();
    assert_eq!(result.x, 20.0);
    assert_eq!(result.y, 75.0);

    let result = value1.clone() + &value2;
    assert_eq!(result.x, 20.0);
    assert_eq!(result.y, 75.0);

    let result = &value1 + value2.clone();
    assert_eq!(result.x, 20.0);
    assert_eq!(result.y, 75.0);

    let result = &value1 + &value2;
    assert_eq!(result.x, 20.0);
    assert_eq!(result.y, 75.0);
}

#[test]
fn vector2_sub() {
    let value1 = Vector2 { x: 5.0, y: 50.0 };
    let value2 = Vector2 { x: 15.0, y: 20.0 };

    let result = value1.clone() - value2.clone();
    assert_eq!(result.x, -10.0);
    assert_eq!(result.y, 30.0);

    let result = value1.clone() - &value2;
    assert_eq!(result.x, -10.0);
    assert_eq!(result.y, 30.0);

    let result = &value1 - value2.clone();
    assert_eq!(result.x, -10.0);
    assert_eq!(result.y, 30.0);

    let result = &value1 - &value2;
    assert_eq!(result.x, -10.0);
    assert_eq!(result.y, 30.0);
}
