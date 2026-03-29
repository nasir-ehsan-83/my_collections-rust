/// Sorts a slice in-place using the shell sort algorithm.
///
/// Shell sort is an optimization of insertion sort that allows the exchange of 
/// items that are far apart. The algorithm uses a sequence of gaps to create 
/// sub-lists of elements, which are then individually sorted using insertion sort.
///
/// # Performance
///
/// * **Time Complexity**: $O(n^2)$ (worst case), $O(n \log^2 n)$ (average case depending on gap sequence).
/// * **Space Complexity**: $O(1)$ (in-place).
/// * **Stability**: This implementation is unstable (may change the relative order of equal elements).
///
/// # Examples
///
/// ```
/// use my_collections::sort::shell_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// shell_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
///
/// let mut strings = ["banana", "apple", "cherry"];
/// shell_sort(&mut strings);
/// assert_eq!(strings, ["apple", "banana", "cherry"]);
/// ```
pub fn shell_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    // Initialize the gap sequence (Shell's original: n/2, n/4, ...)
    let mut gap = len / 2;

    while gap > 0 {
        // Perform a gapped insertion sort for this gap size
        for i in gap..len {
            let mut j = i;

            // Shift elements until the correct location for array[i] is found
            while j >= gap && array[j - gap] > array[j] {
                array.swap(j - gap, j);
                j -= gap;
            }
        }

        // Reduce the gap for the next pass
        gap /= 2;
    }
}
