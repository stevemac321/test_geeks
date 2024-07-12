pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32
}
pub fn rect_intersect(r1: &Rect, r2: &Rect) ->bool {
    if r1.right < r2.left || r2.right < r1.left {
        return false;
    }
    
    if r1.bottom > r2.top || r2.bottom > r1.top {
        return false;
    }
    true  
}