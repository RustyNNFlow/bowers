use super::bbox::IntBoxLike;

pub trait BinaryMaskLike {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn area(&self) -> usize;
    fn iou(&self, other: &Self) -> f64;
    fn get_sub_mask(&self, roi: &impl IntBoxLike) -> Self;
}

pub struct BinaryMask {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>,
}

impl BinaryMaskLike for BinaryMask {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn area(&self) -> usize {
        self.data.iter().filter(|&&x| x).count()
    }

    fn iou(&self, other: &Self) -> f64 {
        let mut intersection = 0;
        let mut union = 0;
        for i in 0..self.data.len() {
            if self.data[i] || other.data[i] {
                union += 1;
            }
            if self.data[i] && other.data[i] {
                intersection += 1;
            }
        }
        intersection as f64 / union as f64
    }

    fn get_sub_mask(&self, roi: &impl IntBoxLike) -> Self {
        let mut data = Vec::with_capacity(roi.width() * roi.height());
        for y in roi.iy1()..roi.iy2() {
            for x in roi.ix1()..roi.ix2() {
                data.push(self.data[y as usize * self.width + x as usize]);
            }
        }
        BinaryMask {
            width: roi.width(),
            height: roi.height(),
            data,
        }
    }
}
