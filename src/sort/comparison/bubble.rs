/// Sorts a slice in-place using the bubble sort algorithm.
///
/// Bubble sort is a simple sorting algorithm that repeatedly steps through the list,
/// compares adjacent elements and swaps them if they are in the wrong order. 
/// The pass through the list is repeated until the list is sorted.
///
/// # Performance
///
/// * **Time Complexity**: $O(n^2)$ (worst/average case), $O(n)$ (best case/already sorted).
/// * **Space Complexity**: $O(1)$ (in-place).
/// * **Stability**: This implementation is stable (preserves the relative order of equal elements).
///
/// # Examples
///
/// ```
/// use my_collections::sort::bubble_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// bubble_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
///
/// let mut strings = ["banana", "apple", "cherry"];
/// bubble_sort(&mut strings);
/// assert_eq!(strings, ["apple", "banana", "cherry"]);
/// ```
pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    for i in (1..len).rev() {
        let mut swapped = false;

        for j in 0..i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        // Optimization: If no elements were swapped, the array is sorted.
        if !swapped {
            break;
        }
    }
}
