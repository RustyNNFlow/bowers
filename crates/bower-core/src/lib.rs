pub mod data;
pub mod image_processing;

pub trait WidthAHeight {
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn iwidth(&self) -> u32{
        self.width() as u32
    }
    fn iheight(&self) -> u32 {
        self.height() as u32
    }
}
