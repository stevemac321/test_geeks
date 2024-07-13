pub fn insertion_sort<T: PartialOrd + PartialEq>(arr: &mut [T]) {
    if arr.len() < 1 {
        println!("bad array");
        return;
    }

    for left in 0..arr.len() - 1 {
        let mut right: usize = left + 1;
        while right > 0 && arr[right] < arr[right - 1] {
            arr.swap(right, right - 1);
            right -= 1;
        }
    }
}
/*
arr: [6, 8, 10, 1, 9]
assumtion: arr len > 0
left = 0, right = 1

General algorithm:
left iterates arr until arr.len - 1
in each iteration of left, right iterates each element to the right
of left while arr[right] < arr.len.
at each of right's visit of an element, that element is moved into its 
proper place in the sorted partition of the array. To start, arr[0] is the
"sorted" partition.

so in the case given above: arr: [6, 8, 10, 1, 9] with left = 0, 
right - 1,left stays at 0, while right iterates and does not have to
move an element until arr[3] = 1.  

Then while a[right] < arr[right-1] && right > 0, swap arr[right] 
with arr[right-1]

Iterate left and repeat while left < arr.len

*/