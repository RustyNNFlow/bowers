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

macro_rules! impl_point_like_arrays {
    ($($ty: ty),*) => {
        $(
            impl PointLike for [$ty;2] {
                fn x(&self) -> f64 {
                    self[0] as f64
                }
                fn y(&self) -> f64 {
                    self[1] as f64
                }
            }
        )*
    }
}

impl_point_like_arrays!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
