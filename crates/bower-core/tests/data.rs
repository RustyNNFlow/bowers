use bower_core::data::{BoxLike, IntBoxLike, BinaryMask, BinaryMaskLike, IntPointLike, PointLike, Polygon, Bbox};
use bower_core::WidthAHeight;

#[test]
fn test_bbox() {
    let bbox = [1.0, 2.0, 3.0, 4.0];
    assert_eq!(bbox.x1(), 1.0);
    assert_eq!(bbox.y1(), 2.0);
    assert_eq!(bbox.x2(), 3.0);
    assert_eq!(bbox.y2(), 4.0);
    let bbox = (1.0, 2.0, 3.0, 5.0);
    assert_eq!(bbox.width(), 2.0);
    assert_eq!(bbox.iheight(), 3);
    let bbox = [1, 2, 3, 4];
    assert_eq!(bbox.ix1(), 1);
    let box_ = Bbox::new(1, 2, 3, 4);
    assert_eq!(box_.x1(), 1.0);
}

#[test]
fn test_iou() {
    let bbox_1 = [0, 0, 10, 10];
    let bbox_2 = [5, 5, 10, 15];
    let iou = bbox_1.iou(&bbox_2);
    assert_eq!(iou, 0.2);
}

#[test]
fn test_point() {
    let point = [1.0, 2.0];
    assert_eq!(point.x(), 1.0);
    let point = (1.0, 2.0);
    assert_eq!(point.x(), 1.0);
    let point = [1, 2];
    assert_eq!(point.x(), 1.0);
    let point = [1, 2];
    assert_eq!(point.ix(), 1);
    let point = [1.0, 2.0];
    assert_eq!(point.y(), 2.0);
}

#[test]
fn test_polygon() {
    let poly = Polygon::new(vec![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
    assert_eq!(poly.iwidth(), 5);
    assert_eq!(poly.iheight(), 5);
}

#[test]
fn test_mask() {
    let mask = BinaryMask::new(
        vec![true, true, false, true, true, false, false, false, false],
        3,
        3,
    );
    assert_eq!(mask.area(), 4);
    assert_eq!(mask.iwidth(), 3);
    assert_eq!(mask.iheight(), 3);
}
