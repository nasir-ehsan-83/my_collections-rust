/// Sorts a slice in-place using the quick sort algorithm.
///
/// Quick sort selects a pivot element and partitions the array into elements
/// less than the pivot and greater than the pivot, recursively sorting them.
///
/// # Performance
///
/// * **Time Complexity**: O(n log n) (average), O(n²) (worst case)
/// * **Space Complexity**: O(log n)
/// * **Stability**: Unstable
///
/// # Examples
///
/// ```
/// let mut numbers = [4, 2, 5, 1, 3];
/// quick_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
pub fn quick_sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    quick_sort_rec(array);
}

fn quick_sort_rec<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    quick_sort_rec(left);
    quick_sort_rec(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = len - 1;
    let mut i = 0;

    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}