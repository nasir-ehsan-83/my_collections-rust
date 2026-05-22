use super::merge::merge;
/// # Timsort
///
/// A **hybrid** sorting algorithm derived from merge sort and insertion sort.
/// It identifies small ordered subsequences (runs) and merges them efficiently,
/// making it highly effective for real-world data.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n) | Linear time when the input is already sorted. |
/// | **Average Time** | O(n log n) | Highly efficient on many types of real-world data. |
/// | **Worst Time** | O(n log n) | Guaranteed logarithmic time for merging runs. |
/// | **Space Complexity** | O(n) | Requires auxiliary space for the merging process. |
///
/// ## Properties
///
/// * **Stable:** Maintains the relative order of elements with equal values.
/// * **Adaptive:** Performance improves significantly on partially sorted data.
/// * **Not-In-place:** Requires additional memory for temporary storage during merges.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::tim_sort;
///
/// let mut numbers = [5, 1, 9, 3, 7, 4, 8, 2, 6];
/// tim_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
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