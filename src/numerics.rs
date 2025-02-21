use windows_numerics::{Vector2, Vector3};

pub trait FromVector2 {
    type Output;
    fn from_vector2(value: Vector2, z: f32) -> Self::Output;
}

impl FromVector2 for Vector3 {
    type Output = Self;
    fn from_vector2(value: Vector2, z: f32) -> Self {
        Vector3::new(value.X, value.Y, z)
    }
}
