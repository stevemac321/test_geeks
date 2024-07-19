
pub fn jumping_numbers(x: u32) ->bool {
    if x < 10 {
        return true;
    }
    let mut temp: u32 = x;
    let mut last: u32 = temp % 10;
    temp /= 10;
    while temp > 0 {
        let cur: u32 = temp % 10;
        if (cur as i32 - last as i32).abs() != 1 {
            return false;
        }
        else {
            last = cur;
            temp /= 10;
        }
    }
    true
}
    