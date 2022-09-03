use image::{RgbImage};
use bower_core::image_processing::crop::Cropable;
use bower_core::WidthAHeight;

#[test]
fn test_image_rs_crop(){
    let image = RgbImage::new(10, 10);
    let roi = [1, 1, 3, 3];
    let cropped_image = image.crop(&roi);
    assert_eq!(WidthAHeight::iwidth(&cropped_image), 2);

    let sub_image = image.center_crop(5, 5);
    assert_eq!(WidthAHeight::iwidth(&sub_image), 5);
} 