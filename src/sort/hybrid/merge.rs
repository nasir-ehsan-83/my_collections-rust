/// Sorts a slice using the merge sort algorithm.
///
/// Merge sort divides the array into halves, recursively sorts them,
/// and merges the sorted halves.
///
/// # Performance
///
/// * **Time Complexity**: O(n log n)
/// * **Space Complexity**: O(n)
/// * **Stability**: Stable
///
/// # Examples
///
/// ```
/// let mut numbers = [4, 2, 5, 1, 3];
/// merge_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
pub fn merge_sort<T: Ord + Clone + Copy>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut array[..mid]);
    merge_sort(&mut array[mid..]);

    let mut merged = array.to_vec();
    merge(&array[..mid], &array[mid..], &mut merged[..]);
    array.copy_from_slice(&merged);
}

pub fn merge<T: Ord + Clone>(left: &[T], right: &[T], result: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i].clone();
            i += 1;
        } else {
            result[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}