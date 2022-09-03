use crate::WidthAHeight;

pub trait ImageLike: WidthAHeight{
    type Pixel;
    fn channels(&self) -> u8;
}