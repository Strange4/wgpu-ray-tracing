use std::ops::{Add, Div, Mul, Sub};

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

type Point3<T> = Vec3<T>;

impl<T> Vec3<T>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn dot(&self, v: Self) -> T {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    #[inline]
    pub fn cross(&self, v: Self) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    #[inline]
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Vec3<f32> {
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn unit(self) -> Self {
        let division = 1.0 / self.length();
        self * division
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
