pub trait BoxLike {
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
    fn width(&self) -> f64 {
        self.x2() - self.x1()
    }
    fn height(&self) -> f64 {
        self.y2() - self.y1()
    }
}

pub trait IntBoxLike {
    fn ix1(&self) -> isize;
    fn iy1(&self) -> isize;
    fn ix2(&self) -> isize;
    fn iy2(&self) -> isize;
    fn iwidth(&self) -> usize {
        (self.ix2() - self.ix1()) as usize
    }
    fn iheight(&self) -> usize {
        (self.iy2() - self.iy1()) as usize
    }
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

    pub fn as_arr(self) -> [f64; 4] {
        self.0
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let box_ = Bbox::new(1, 2, 3, 4);
        assert_eq!(box_.x1(), 1.0);
        let a = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(a.x1(), 1.0);
        let box_2 = [1, 2, 3, 4];
        assert_eq!(box_2.x1(), 1.0);
        assert_eq!(box_2.ix1(), 1);
    }

    #[test]
    fn test_iou() {
        let bbox_1 = [0, 0, 10, 10];
        let bbox_2 = [5, 5, 10, 15];
        let iou = bbox_1.iou(&bbox_2);
        assert_eq!(iou, 0.2);
    }
}
