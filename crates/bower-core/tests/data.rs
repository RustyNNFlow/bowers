use bower_core::data::{
    bbox::{BoxLike, IntBoxLike},
    mask::{BinaryMask, BinaryMaskLike},
    point::{IntPointLike, PointLike},
    polygon::Polygon,
};

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
    assert_eq!(poly.width(), 5);
    assert_eq!(poly.height(), 5);
}

#[test]
fn test_mask() {
    let mask = BinaryMask::new(
        vec![true, true, false, true, true, false, false, false, false],
        3,
        3,
    );
    assert_eq!(mask.area(), 4);
    assert_eq!(mask.width(), 3);
    assert_eq!(mask.height(), 3);
}
