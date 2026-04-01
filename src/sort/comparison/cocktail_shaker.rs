/// Sorts a slice in-place using the cocktail shaker sort algorithm.
///
/// Cocktail shaker sort is a bidirectional bubble sort. It moves the largest
/// element to the end and the smallest element to the beginning in each pass.
///
/// # Performance
///
/// * **Time Complexity**: $O(n^2)$ (worst and average), $O(n)$ (best).
/// * **Space Complexity**: $O(1)$.
/// * **Stability**: This implementation is **stable**.
///
/// # Examples
///
/// ```
/// use my_collections::sort::cocktail_shaker_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// cocktail_shaker_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
pub fn cocktail_shaker_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    let mut start = 0;
    let mut end = len - 1;
    let mut swapped = true;

    while swapped {
        swapped = false;

        // Forward pass
        for i in start..end {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        // Backward pass
        for i in (start..end).rev() {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }

        start += 1;
    }
}
