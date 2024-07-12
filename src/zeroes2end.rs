pub fn zeroes2end(arr: &mut [i32]) ->usize {
    let mut newlen: usize = 0;

    for i in 0..arr.len() {
        if arr[i] != 0 {
            arr[newlen] = arr[i];
            newlen += 1;
        }
    }
    for i in newlen..arr.len() {
        arr[i] = 0;
    }
    newlen
}