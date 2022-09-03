use crate::data::{ IntBoxLike};
use crate::WidthAHeight;

fn get_center_crop_roi(source: &impl WidthAHeight, target_width: u32, target_height: u32)->[u32; 4]{
    let source_width = source.iwidth();
    let source_height = source.iheight();
    let (cx, cy) = (source_width / 2, source_height / 2);
    let (x1, y1) = (cx - target_width / 2, cy - target_height / 2);
    let (x2, y2) = (x1 + target_width, y1 + target_height);
    [x1, y1, x2, y2]
}

pub trait Cropable: WidthAHeight+Sized {
    fn crop(&self, roi: &impl IntBoxLike) -> Self;
    fn crop_mut(&mut self, roi: &impl IntBoxLike);
    fn center_crop(&self, target_width: u32, target_height: u32) -> Self {
        let roi = get_center_crop_roi(self, target_width, target_height);
        self.crop(&roi)
    }
    fn center_crop_mut(&mut self, target_width: u32, target_height: u32){
        let roi = get_center_crop_roi(self,target_width, target_height);
        self.crop_mut(&roi);
    }
}