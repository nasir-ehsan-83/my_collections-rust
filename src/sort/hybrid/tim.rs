use super::merge::merge;

/// Sorts a slice using a simplified TimSort algorithm.
///
/// Combines insertion sort for small runs and merge sort for merging.
///
/// # Performance
///
/// * **Time Complexity**: O(n log n)
/// * **Space Complexity**: O(n)
/// * **Stability**: Stable
pub fn tim_sort<T: Ord + Clone>(array: &mut [T]) {
    const RUN: usize = 32;
    let len = array.len();

    for i in (0..len).step_by(RUN) {
        insertion_sort(&mut array[i..(i + RUN).min(len)]);
    }

    let mut size = RUN;
    while size < len {
        for left in (0..len).step_by(2 * size) {
            let mid = (left + size).min(len);
            let right = (left + 2 * size).min(len);
            if mid < right {
                merge(&array[left..mid], &array[mid..right], &mut array[left..right].to_vec());
            }
        }
        size *= 2;
    }
}

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}