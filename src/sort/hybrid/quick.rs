/// # Quick Sort
///
/// A **divide-and-conquer** sorting algorithm that partitions a slice into
/// sub-arrays based on a pivot element. It recursively sorts the partitions
/// by placing the pivot in its final sorted position.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n log n) | Occurs when the pivot consistently divides the slice in half. |
/// | **Average Time** | O(n log n) | Highly efficient for random distributions. |
/// | **Worst Time** | O(n²) | Occurs with poor pivot selection on sorted or reversed data. |
/// | **Space Complexity** | O(log n) | Space used for the recursive call stack. |
///
/// ## Properties
///
/// * **Not-Stable:** The partitioning process does not preserve the order of equal elements.
/// * **Not-adaptive:** Performance typically degrades on already sorted arrays (unless optimized).
/// * **In-place:** Sorts the elements within the original slice using swaps.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::quick_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// quick_sort(&mut numbers);
///
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