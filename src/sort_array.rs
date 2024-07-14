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
pub fn bubble_sort<T: PartialOrd + PartialEq>(arr: &mut [T]) {
    if arr.len() < 1 {
        println!("bad array");
        return;
    }

    for sorted_end in 0..arr.len() - 1 {
        for current in 0..arr.len() - 1 - sorted_end {
            if arr[current] > arr[current + 1] {
                arr.swap(current, current + 1);
            }
        }
    }
}

fn partition<T: PartialOrd + PartialEq>(arr: &mut [T], low: usize, high: usize) ->usize{
    let pivot = high;
    let mut left = low;

    for scanner in low..high {
        if arr[scanner] <= arr[pivot] {
            arr.swap(left,scanner);
            left += 1;
        }
    }
    arr.swap(left, high);
    left
}

fn inner_qsort<T: PartialOrd + PartialEq>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(arr, low, high);
        if pivot > 0 {
            inner_qsort(arr, low, pivot - 1); // Corrected to pivot - 1
        }
        inner_qsort(arr, pivot + 1, high); // Corrected to pivot + 1
    }
}
pub fn quick_sort<T: PartialOrd + PartialEq>(arr: &mut [T]) {
    inner_qsort(arr, 0, arr.len() - 1);
}

/*
Insertion Sort
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
-----------------------------------------------------
Bubble Sort:
Bubble sort is a simple sorting algorithm that repeatedly steps through the list, 
compares adjacent elements, and swaps them if they are in the wrong order. 
Here’s the general algorithm:

1. **Initialization**:
   - Start with the first element of the array.

2. **Comparisons and Swaps**:
   - Compare the current element with the next element.
   - If the current element is greater than the next element, swap them.

3. **Iterative Process**:
   - Move to the next element and repeat the comparison and swapping process.
   - Continue this process for each element in the array.

4. **Passes**:
   - After completing one full pass through the array, the largest element is 
   guaranteed to be at the end of the array.
   - Repeat the process for the remaining elements, excluding the last element
    which is already sorted.

5. **Termination**:
   - Continue making passes through the array until no swaps are needed, indicating that the array is sorted.

### Key Points:
- **Multiple Passes**: The algorithm requires multiple passes through the array.
- **Adjacent Comparisons**: It compares adjacent elements and swaps them if necessary.
- **Gradual Sorting**: With each pass, the next largest element is bubbled to its 
correct position at the end of the array.
- **Inefficient for Large Data Sets**: Bubble sort is not suitable for large 
datasets as its average and worst-case complexity are both \(O(n^2)\).

### Efficiency:
- **Best Case**: \(O(n)\) when the array is already sorted.
- **Average and Worst Case**: \(O(n^2)\) due to the nested loops required for comparisons and swaps.
---------------------------------------------------------------------
Quick Sort
#include <iostream>
template <typename T, const size_t N> constexpr size_t __countof(T(&)[N]) {
    return N;
}

void swap(int a[], int left, int right) {
    int temp = a[right];
    a[right] = a[left];
    a[left] = temp;
}

int partition(int a[], int low, int high) {
    int pivot = a[high];
    int left = low;

    for (int scanner = low; scanner < high; scanner++) {
        if (a[scanner] < pivot) {
            swap(a, left, scanner);
            left++;
        }
    }
    // Swap the pivot to its correct position
    swap(a, left, high);
    return left;
}

void internal_qsort(int a[], int low, int high) {
    if (low < high) {
        int pivot = partition(a, low, high);
        internal_qsort(a, low, pivot - 1); // Recursively sort left partition
        internal_qsort(a, pivot + 1, high); // Recursively sort right partition
    }
}

void qsort(int a[], int len) {
    internal_qsort(a, 0, len - 1);
}

int main() {
    int a[] = { 33, 77, 1, 8, 0, 3, 5, 1, 79, 33, 9 };
    size_t len = __countof(a);
    qsort(a, len);
    for (auto& i : a)
        std::cout << i << "\n";
}

*/