use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::location::Location;

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Copy, Clone, Ord)]
pub struct Point2d<T>
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
}

impl<T> Point2d<T>
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
    pub fn new(x: T, y: T) -> Point2d<T> {
        Point2d { x, y }
    }
}

impl<T: Add<Output = T>> Add for Point2d<T>
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
        }
    }
}

impl<T> AddAssign for Point2d<T>
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
    }
}

impl<T: Sub<Output = T>> Sub for Point2d<T>
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
        }
    }
}

impl<T> SubAssign for Point2d<T>
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
    }
}

impl<T: Mul<Output = T>> Mul for Point2d<T>
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
        }
    }
}

impl<T> MulAssign for Point2d<T>
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
    }
}

impl<T: Div<Output = T>> Div for Point2d<T>
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
        }
    }
}

impl<T> DivAssign for Point2d<T>
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
    }
}

impl<T> Location for Point2d<T>
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

    fn manhattan_distance_to(&self, other: &Point2d<T>) -> T {
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

        relative_x + relative_y
    }

    fn distance_to(&self, other: &Point2d<T>) -> f64 {
        let relative_x = other.x - self.x;
        let relative_y = other.y - self.y;

        let temp = (relative_x * relative_x + relative_y * relative_y).into();

        temp.sqrt()
    }

    fn add(&self, other: &Point2d<T>) -> Point2d<T> {
        let new_x = self.x + other.x;
        let new_y = self.y + other.y;

        Point2d::new(new_x, new_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    const ORIGIN_POINT: Point2d<i32> = Point2d { x: 0, y: 0 };

    #[test]
    fn test_manhattan_distance_to() {
        let point = Point2d::new(-5, 5);

        let expected = 10;

        let result = ORIGIN_POINT.manhattan_distance_to(&point);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_to() {
        let point = Point2d::new(3, 4);

        let expected = 5.0;

        let result = ORIGIN_POINT.distance_to(&point);

        assert!((result - expected).abs() < EPSILON);
    }

    #[test]
    fn test_add() {
        let first = Point2d::new(3, 4);
        let second = Point2d::new(5, -1);

        let expected = Point2d::new(8, 3);

        let result = first + second;

        assert_eq!(result, expected);
    }
}
