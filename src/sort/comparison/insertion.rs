/// Sorts a slice in-place using the insertion sort algorithm.
///
/// Insertion sort is a simple sorting algorithm that builds the final sorted array
/// one item at a time. It is much less efficient on large lists than more advanced
/// algorithms like quicksort, heapsort, or merge sort.
///
/// # Performance
///
/// * **Time Complexity**: $O(n^2)$ (worst/average case), $O(n)$ (best case/nearly sorted).
/// * **Space Complexity**: $O(1)$ (in-place).
/// * **Stability**: This implementation is stable (preserves the relative order of equal elements).
///
/// # Examples
///
/// ```
/// use my_collections::sort::insertion_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// insertion_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
///
/// let mut strings = ["banana", "apple", "cherry"];
/// insertion_sort(&mut strings);
/// assert_eq!(strings, ["apple", "banana", "cherry"]);
/// ```
pub fn insertion_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    for i in 1..array.len() {
        let key = array[i].clone();
        let mut j = i;

        // Shift elements that are greater than the key to the right
        while j > 0 && array[j - 1] > key {
            array[j] = array[j - 1].clone();
            j -= 1;
        }

        array[j] = key;
    }
}
