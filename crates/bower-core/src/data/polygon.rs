use super::{bbox::IntBoxLike, mask::BinaryMaskLike, point::PointLike};

pub struct Polygon<T: PointLike>(Vec<T>);

impl<T: PointLike + Clone> Polygon<T> {
    pub fn new(points: Vec<T>) -> Self {
        Polygon(points)
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.0.clone()
    }

    pub fn as_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T: PointLike> BinaryMaskLike for Polygon<T> {
    fn width(&self) -> usize {
        let max_x = self.0.iter().map(|p| p.x()).fold(f64::NAN, f64::max) as usize;
        let min_x = self.0.iter().map(|p| p.x()).fold(f64::NAN, f64::min) as usize;
        // TODO check add 1 ?
        max_x - min_x + 1
    }
    fn height(&self) -> usize {
        let max_x = self.0.iter().map(|p| p.y()).fold(f64::NAN, f64::max) as usize;
        let min_x = self.0.iter().map(|p| p.y()).fold(f64::NAN, f64::min) as usize;
        max_x - min_x + 1
    }
    fn area(&self) -> usize {
        todo!()
    }
    fn iou(&self, other: &Self) -> f64 {
        todo!()
    }
    fn get_sub_mask(&self, roi: &impl IntBoxLike) -> Self {
        todo!()
    }
}
