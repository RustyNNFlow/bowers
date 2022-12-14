use crate::WidthAHeight;
use super::bbox::IntBoxLike;

pub trait BinaryMaskLike: WidthAHeight {
    fn area(&self) -> usize;
    fn iou(&self, other: &Self) -> f64;
    fn get_sub_mask(&self, roi: &impl IntBoxLike) -> Self;
}

pub struct BinaryMask {
    pub data: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

impl BinaryMask {
    pub fn new(data: Vec<bool>, width: u32, height: u32) -> Self {
        BinaryMask {
            width,
            height,
            data,
        }
    }
}

impl WidthAHeight for BinaryMask{
    fn width(&self) -> f64 {
        self.width as f64
    }

    fn height(&self) -> f64 {
        self.height as f64
    }
}

impl BinaryMaskLike for BinaryMask {

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
        let mut data = Vec::with_capacity((roi.iwidth() * roi.iheight()) as usize);
        for y in roi.iy1()..roi.iy2() {
            for x in roi.ix1()..roi.ix2() {
                data.push(self.data[y as usize * self.width as usize + x as usize]);
            }
        }
        BinaryMask {
            width: roi.iwidth(),
            height: roi.iheight(),
            data,
        }
    }
}
