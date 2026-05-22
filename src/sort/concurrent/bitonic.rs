/// # Bitonic Sort
///
/// A **hybrid** sorting algorithm and a **comparison network** model that builds
/// a bitonic sequence through recursive sorting and merging. It is highly
/// optimized for parallel processing.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(log² n) | Parallel complexity (with n/2 processors). |
/// | **Average Time** | O(n log² n) | Sequential complexity on a single processor. |
/// | **Worst Time** | O(n log² n) | Performance is independent of initial data order. |
/// | **Space Complexity** | O(log n) | Space used by the recursive call stack. |
///
/// ## Properties
///
/// * **Not-Stable:** Does not maintain the relative order of identical elements.
/// * **Not-adaptive:** Comparison steps are predefined (data-independent).
/// * **In-place:** Can be implemented to sort within the original array boundaries.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::bitonic_sort;
///
/// // Best performance with power-of-two lengths
/// let mut numbers = [10, 30, 11, 20, 4, 33, 21, 5];
/// bitonic_sort(&mut numbers);
///
/// assert_eq!(numbers, [4, 5, 10, 11, 20, 21, 30, 33]);
/// ```
pub fn bitonic_sort<T: Ord>(array: &mut [T]) {
    //let len = array.len();
    bitonic_sort_rec(array, true);
}

fn bitonic_sort_rec<T: Ord>(arr: &mut [T], ascending: bool) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    bitonic_sort_rec(&mut arr[..mid], true);
    bitonic_sort_rec(&mut arr[mid..], false);
    bitonic_merge(arr, ascending);
}

fn bitonic_merge<T: Ord>(arr: &mut [T], ascending: bool) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    for i in 0..mid {
        if (arr[i] > arr[i + mid]) == ascending {
            arr.swap(i, i + mid);
        }
    }

    bitonic_merge(&mut arr[..mid], ascending);
    bitonic_merge(&mut arr[mid..], ascending);
}