extern crate test_geeks;
use test_geeks::rect_intersect::rect_intersect;
use test_geeks::rect_intersect::Rect;


#[test]
fn test_rect_intersect() {
    {
        let r1 = Rect{left: 0, right: 4, top: 4, bottom: 0};
        let r2 = Rect{left: 2, right: 6, top: 6, bottom: 2};
        let ret: bool = rect_intersect(&r1,&r2);
        assert_eq!(ret,true);
    }
    {
        let r1 = Rect{left: 0, right: 4, top: 4, bottom: 0};
        let r2 = Rect{left: 100, right: 200, top: 20, bottom: 10};
        let ret: bool = rect_intersect(&r1,&r2);
        assert_eq!(ret,false);
    }

}