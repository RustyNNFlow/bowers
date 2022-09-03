
use image::{ImageBuffer, Pixel, imageops};
use crate::data::{ImageLike, IntBoxLike};
use crate::image_processing::crop::Cropable;
use crate::WidthAHeight;

impl<P:Pixel> WidthAHeight for ImageBuffer<P, Vec<P::Subpixel>>{
    fn width(&self) -> f64{
        ImageBuffer::width(self) as f64
    }
    fn height(&self) -> f64{
        ImageBuffer::height(self) as f64
    }
}

impl<P:Pixel> ImageLike for ImageBuffer<P, Vec<P::Subpixel>> {
    type Pixel = P;
    fn channels(&self) -> u8{
        P::CHANNEL_COUNT
    }
}

impl <P:Pixel> Cropable for ImageBuffer<P, Vec<P::Subpixel>> where P:'static, P::Subpixel: 'static{
    fn crop(&self, roi: &impl IntBoxLike) -> Self{
        let x1 = roi.ix1() as u32;
        let y1 = roi.iy1() as u32;
        let width = roi.iwidth() as u32;
        let height = roi.iheight() as u32;
        let sub_image = imageops::crop_imm(self, x1, y1, width, height);
        sub_image.to_image()
    }

    fn crop_mut(&mut self, roi: &impl IntBoxLike){
        let x1 = roi.ix1() as u32;
        let y1 = roi.iy1() as u32;
        let width = roi.iwidth() as u32;
        let height = roi.iheight() as u32;
        let sub_image = imageops::crop(self, x1, y1, width, height);
        *self = sub_image.to_image();
    }
}