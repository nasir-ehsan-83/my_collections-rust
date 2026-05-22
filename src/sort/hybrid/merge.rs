/// # Merge Sort
///
/// A **divide-and-conquer** sorting algorithm that recursively partitions a slice
/// into halves, sorts each half individually, and then merges them back together.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n log n) | Always divides the array, regardless of order. |
/// | **Average Time** | O(n log n) | Highly predictable and consistent performance. |
/// | **Worst Time** | O(n log n) | Guaranteed performance even on worst-case inputs. |
/// | **Space Complexity** | O(n) | Requires auxiliary space for the merging process. |
///
/// ## Properties
///
/// * **Stable:** Preserves the relative order of elements with equal values.
/// * **Not-adaptive:** The number of comparisons remains the same for any input.
/// * **Not-In-place:** Typically requires an additional buffer to merge elements.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::merge_sort;
///
/// let mut numbers = [38, 27, 43, 3, 9, 82, 10];
/// merge_sort(&mut numbers);
///
/// assert_eq!(numbers, [3, 9, 10, 27, 38, 43, 82]);
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