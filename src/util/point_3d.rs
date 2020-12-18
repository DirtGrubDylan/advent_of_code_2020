use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::location::Location;

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Copy, Clone, Ord)]
pub struct Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Point3d<T> {
        Point3d { x, y, z }
    }
}

impl<T: Add<Output = T>> Add for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Sub<Output = T>> Sub for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> SubAssign for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Mul<Output = T>> Mul for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> MulAssign for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: Div<Output = T>> Div for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> DivAssign for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T> Location for Point3d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + Ord
        + Into<f64>
        + From<u8>
        + Copy,
{
    type ValueOutput = T;

    fn manhattan_distance_to(&self, other: &Point3d<T>) -> T {
        let relative_x = if other.x < self.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let relative_y = if other.y < self.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        let relative_z = if other.z < self.z {
            self.z - other.z
        } else {
            other.z - self.z
        };

        relative_x + relative_y + relative_z
    }

    fn distance_to(&self, other: &Point3d<T>) -> f64 {
        let relative_x = other.x - self.x;
        let relative_y = other.y - self.y;
        let relative_z = other.z - self.z;

        let temp =
            (relative_x * relative_x + relative_y * relative_y + relative_z * relative_z).into();

        temp.sqrt()
    }

    fn add(&self, other: &Point3d<T>) -> Point3d<T> {
        *self + *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    const ORIGIN_POINT: Point3d<i32> = Point3d { x: 0, y: 0, z: 0 };

    #[test]
    fn test_manhattan_distance_to() {
        let point = Point3d::new(-5, 5, 3);

        let result = ORIGIN_POINT.manhattan_distance_to(&point);

        let expected = 13;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_to() {
        let point = Point3d::new(3, 4, 12);

        let expected = 13.0;

        let result = ORIGIN_POINT.distance_to(&point);

        assert!((result - expected).abs() < EPSILON);
    }

    #[test]
    fn test_add() {
        let first = Point3d::new(3, 4, -3);
        let second = Point3d::new(5, -1, 3);

        let result = first + second;

        let expected = Point3d::new(8, 3, 0);

        assert_eq!(result, expected);
    }
}
