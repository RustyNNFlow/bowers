use std::ops::{Add, Sub};

pub trait BoxLike {
    type Item: Sub<Output = Self::Item>;
    fn x1(&self) -> Self::Item;
    fn y1(&self) -> Self::Item;
    fn x2(&self) -> Self::Item;
    fn y2(&self) -> Self::Item;
    fn width(&self) -> Self::Item {
        self.x2() - self.x1()
    }
    fn height(&self) -> Self::Item {
        self.y2() - self.y1()
    }
}

pub struct Bbox<T: Add<Output = T> + Sub<Output = T> + Copy = f64> {
    pub x1: T,
    pub y1: T,
    pub x2: T,
    pub y2: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Copy> Bbox<T> {
    pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        Bbox { x1, y1, x2, y2 }
    }

    pub fn from_arr(arr: &[T; 4]) -> Self {
        Bbox {
            x1: arr[0],
            y1: arr[1],
            x2: arr[2],
            y2: arr[3],
        }
    }

    // pub fn from_slice(slice: &[T]) -> Self {
    //     Bbox {
    //         x1: slice[0],
    //         y1: slice[1],
    //         x2: slice[2],
    //         y2: slice[3],
    //     }
    // }

    pub fn from_wh(x: T, y: T, w: T, h: T) -> Self {
        Bbox {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn from_other<U: Add<Output = U> + Sub<Output = U> + Copy>(other: &Bbox<U>) -> Self
    where
        U: Into<T>,
    {
        Bbox {
            x1: other.x1.into(),
            y1: other.y1.into(),
            x2: other.x2.into(),
            y2: other.y2.into(),
        }
    }

    // pub fn from_vec(vec: &Vec<T>) -> Self {
    //     Bbox {
    //         x1: vec[0],
    //         y1: vec[1],
    //         x2: vec[2],
    //         y2: vec[3],
    //     }
    // }

    pub fn from_bbox(bbox: impl BoxLike<Item = T>) -> Self {
        Bbox {
            x1: bbox.x1(),
            y1: bbox.y1(),
            x2: bbox.x2(),
            y2: bbox.y2(),
        }
    }

    pub fn to_arr(&self) -> [T; 4] {
        [self.x1, self.y1, self.x2, self.y2]
    }
}

impl<T> BoxLike for Bbox<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    type Item = T;
    fn x1(&self) -> T {
        self.x1
    }
    fn y1(&self) -> T {
        self.y1
    }
    fn x2(&self) -> T {
        self.x2
    }
    fn y2(&self) -> T {
        self.y2
    }
}

macro_rules! impl_arrays {
    ($($ty: ty),*) => {
        $(
            impl BoxLike for [$ty;4] {
                type Item = $ty;
                fn x1(&self) -> $ty {
                    self[0]
                }
                fn y1(&self) -> $ty {
                    self[1]
                }
                fn x2(&self) -> $ty {
                    self[2]
                }
                fn y2(&self) -> $ty {
                    self[3]
                }
            }
        )*
    }
}

impl_arrays!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let result = Bbox::<i16>::from_arr(&[1, 2, 3, 4]);
        let bbox_f32: Bbox<f32> = Bbox::from_other(&result);
        assert_eq!(bbox_f32.x1, 1.0);
        let a = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(a.x1(), 1.0)
    }
}
