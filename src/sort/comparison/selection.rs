/// Sorts a slice in-place using the selection sort algorithm.
///
/// Selection sort divides the input list into two parts: a sorted sublist of items 
/// built up from left to right and a sublist of the remaining unsorted items. 
/// It repeatedly finds the smallest element from the unsorted part and 
/// swaps it with the leftmost unsorted element.
///
/// # Performance
///
/// * **Time Complexity**: $O(n^2)$ (all cases: best, average, and worst).
/// * **Space Complexity**: $O(1)$ (in-place).
/// * **Stability**: This implementation is **unstable** (may change the relative order of equal elements due to long-distance swaps).
///
/// # Examples
///
/// ```
/// use my_collections::sort::selection_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// selection_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
///
/// let mut strings = ["banana", "apple", "cherry"];
/// selection_sort(&mut strings);
/// assert_eq!(strings, ["apple", "banana", "cherry"]);
/// ```
pub fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    for i in 0..len {
        let mut min_index = i;

        // Find the index of the smallest element in the remaining unsorted portion
        for j in (i + 1)..len {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }

        // Swap the found minimum element with the first element of the unsorted portion
        if min_index != i {
            array.swap(i, min_index);
        }
    }
}
