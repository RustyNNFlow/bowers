pub trait PointLike {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

pub trait IntPointLike {
    fn ix(&self) -> isize;
    fn iy(&self) -> isize;
}

impl<T> IntPointLike for T
where
    T: PointLike,
{
    fn ix(&self) -> isize {
        self.x() as isize
    }

    fn iy(&self) -> isize {
        self.y() as isize
    }
}

pub struct Point([f64; 2]);

impl PointLike for Point {
    fn x(&self) -> f64 {
        self.0[0]
    }

    fn y(&self) -> f64 {
        self.0[1]
    }
}

impl<T: Into<f64> + Copy> PointLike for [T; 2] {
    fn x(&self) -> f64 {
        self[0].into()
    }

    fn y(&self) -> f64 {
        self[1].into()
    }
}

impl<T: Into<f64> + Copy> PointLike for (T, T) {
    fn x(&self) -> f64 {
        self.0.into()
    }

    fn y(&self) -> f64 {
        self.1.into()
    }
}
