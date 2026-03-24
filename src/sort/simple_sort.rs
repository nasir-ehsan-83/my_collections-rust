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
