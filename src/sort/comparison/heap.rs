/// # Heap Sort
///
/// An efficient, in-place sorting algorithm that visualizes the array as a binary heap. 
/// It builds a max-heap and repeatedly extracts the maximum element to the end of 
/// the slice, maintaining $O(n \log n)$ performance across all cases.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | $O(n \log n)$ | Consistently fast even in the best case. |
/// | **Average Time** | $O(n \log n)$ | Very reliable for random data distributions. |
/// | **Worst Time** | $O(n \log n)$ | Guaranteed upper bound on performance. |
/// | **Space Complexity** | $O(1)$ | Performs swaps in-place without extra allocations. |
///
/// ## Properties
/// * **Stability:** ❌ **Unstable**. Does not preserve the order of equal elements.
/// * **Adaptive:** No. The heap construction and extraction steps are fixed.
/// * **In-place:** Yes. Transforms the input slice into a heap structure.
///
/// ## Example
/// ```rust
/// use my_collections::sort::heap_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// heap_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```

pub fn heap_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    // Build max heap
    for i in (0..len / 2).rev() {
        sift_down(array, i, len);
    }

    // Extract elements from heap
    for i in (1..len).rev() {
        array.swap(0, i);
        sift_down(array, 0, i);
    }
}

fn sift_down<T: PartialOrd>(array: &mut [T], mut root: usize, end: usize) {
    while root * 2 + 1 < end {
        let child = root * 2 + 1;
        let mut target = child;

        if child + 1 < end && array[child] < array[child + 1] {
            target = child + 1;
        }

        if array[root] < array[target] {
            array.swap(root, target);
            root = target;
        } else {
            break;
        }
    }
}
