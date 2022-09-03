use crate::WidthAHeight;

pub trait BoxLike: WidthAHeight {
    fn x1(&self) -> f64;
    fn y1(&self) -> f64;
    fn x2(&self) -> f64;
    fn y2(&self) -> f64;
    fn iou(&self, other: &Self) -> f64 {
        let x1 = self.x1().max(other.x1());
        let y1 = self.y1().max(other.y1());
        let x2 = self.x2().min(other.x2());
        let y2 = self.y2().min(other.y2());
        let w = (x2 - x1).max(0_f64);
        let h = (y2 - y1).max(0_f64);
        let inter = w * h;
        let area1 = (self.x2() - self.x1()) * (self.y2() - self.y1());
        let area2 = (other.x2() - other.x1()) * (other.y2() - other.y1());
        inter as f64 / (area1 + area2 - inter) as f64
    }
}

pub trait IntBoxLike: WidthAHeight {
    fn ix1(&self) -> isize;
    fn iy1(&self) -> isize;
    fn ix2(&self) -> isize;
    fn iy2(&self) -> isize;
}

impl<T> IntBoxLike for T
where
    T: BoxLike,
{
    fn ix1(&self) -> isize {
        self.x1() as isize
    }

    fn iy1(&self) -> isize {
        self.y1() as isize
    }

    fn ix2(&self) -> isize {
        self.x2() as isize
    }

    fn iy2(&self) -> isize {
        self.y2() as isize
    }
}

pub struct Bbox([f64; 4]);

impl Bbox {
    pub fn new<T: Into<f64>>(x1: T, y1: T, x2: T, y2: T) -> Self {
        Bbox([x1.into(), y1.into(), x2.into(), y2.into()])
    }

    pub fn from_arr<T: Into<f64> + Copy>(arr: &[T; 4]) -> Self {
        Bbox([arr[0].into(), arr[1].into(), arr[2].into(), arr[3].into()])
    }

    pub fn from_wh<T: Into<f64> + Copy>(x: T, y: T, w: T, h: T) -> Self {
        Bbox([x.into(), y.into(), x.into() + w.into(), y.into() + h.into()])
    }

    pub fn from_bbox(bbox: impl BoxLike) -> Self {
        Bbox::new(bbox.x1(), bbox.y1(), bbox.x2(), bbox.y2())
    }

    pub fn to_arr(&self) -> [f64; 4] {
        self.0
    }

    pub fn into_arr(self) -> [f64; 4] {
        self.0
    }
}

impl WidthAHeight for Bbox {
    fn width(&self) -> f64 {
        self.0[2] - self.0[0]
    }

    fn height(&self) -> f64 {
        self.0[3] - self.0[1]
    }
}

impl<T: Into<f64> + Copy> WidthAHeight for [T; 4]{
    fn width(&self) -> f64 {
        self[2].into() - self[0].into()
    }

    fn height(&self) -> f64 {
        self[3].into() - self[1].into()
    }
}

impl<T: Into<f64> + Copy> WidthAHeight for (T, T, T, T) {
    fn width(&self) -> f64 {
        self.2.into() - self.0.into()
    }

    fn height(&self) -> f64 {
        self.3.into() - self.1.into()
    }
}

impl BoxLike for Bbox {
    fn x1(&self) -> f64 {
        self.0[0]
    }

    fn y1(&self) -> f64 {
        self.0[1]
    }

    fn x2(&self) -> f64 {
        self.0[2]
    }

    fn y2(&self) -> f64 {
        self.0[3]
    }
}

impl<T: Into<f64> + Copy> BoxLike for [T; 4] {
    fn x1(&self) -> f64 {
        self[0].into()
    }

    fn y1(&self) -> f64 {
        self[1].into()
    }

    fn x2(&self) -> f64 {
        self[2].into()
    }

    fn y2(&self) -> f64 {
        self[3].into()
    }
}

impl<T: Into<f64> + Copy> BoxLike for (T, T, T, T) {
    fn x1(&self) -> f64 {
        self.0.into()
    }

    fn y1(&self) -> f64 {
        self.1.into()
    }

    fn x2(&self) -> f64 {
        self.2.into()
    }

    fn y2(&self) -> f64 {
        self.3.into()
    }
}
