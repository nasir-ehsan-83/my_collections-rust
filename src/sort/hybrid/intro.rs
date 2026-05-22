/// # Introsort
///
/// A **hybrid** sorting algorithm that provides both fast average performance
/// and optimal worst-case behavior. It begins with Quick Sort and switches to
/// Heap Sort when the recursion depth exceeds a predefined limit.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n log n) | Consistent with quick sort's ideal partitions. |
/// | **Average Time** | O(n log n) | Highly efficient for most general cases. |
/// | **Worst Time** | O(n log n) | Guaranteed by switching to heap sort. |
/// | **Space Complexity** | O(log n) | Space used for the recursive stack. |
///
/// ## Properties
///
/// * **Not-Stable:** Does not maintain the relative order of equal elements.
/// * **Not-adaptive:** Does not significantly speed up for partially sorted data.
/// * **In-place:** Performs sorting within the original memory allocation.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::intro_sort;
///
/// let mut numbers = [42, 32, 33, 52, 37, 47, 51];
/// intro_sort(&mut numbers);
///
/// assert_eq!(numbers, [32, 33, 37, 42, 47, 51, 52]);
/// ```
pub fn intro_sort<T: Ord>(array: &mut [T]) {
    let depth = (array.len() as f64).log2() as usize * 2;
    introsort_rec(array, depth);
}

fn introsort_rec<T: Ord>(arr: &mut [T], depth: usize) {
    if arr.len() <= 1 {
        return;
    }

    if depth == 0 {
        arr.sort(); 
        return;
    }

    let p = partition(arr);
    let (left, right) = arr.split_at_mut(p);
    introsort_rec(left, depth - 1);
    introsort_rec(&mut right[1..], depth - 1);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    // Simple pivot selection: middle element
    let pivot_idx = len / 2;
    
    // Move pivot to the end temporarily
    arr.swap(pivot_idx, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    // Move pivot to its final place
    arr.swap(i, len - 1);
    i
}
