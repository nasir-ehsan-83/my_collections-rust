/// Sorts a slice in-place using the heap sort algorithm.
///
/// Heap sort visualizes the array as a binary heap. It first builds a max-heap, 
/// then repeatedly extracts the maximum element and moves it to the end of the 
/// array, rebuilding the heap with the remaining elements.
///
/// # Performance
///
/// * **Time Complexity**: $O(n \log n)$ (all cases).
/// * **Space Complexity**: $O(1)$ (in-place).
/// * **Stability**: This implementation is unstable.
///
/// # Examples
///
/// ```
/// use my_collections::sort::heap_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// heap_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
///
/// let mut strings = ["banana", "apple", "cherry"];
/// heap_sort(&mut strings);
/// assert_eq!(strings, ["apple", "banana", "cherry"]);
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
