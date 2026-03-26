use std::vec::Vec;
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




/// Sorts a slice in-place using the Merge Sort algorithm.
///
/// Merge Sort is a divide-and-conquer algorithm that recursively splits the array 
/// in half, sorts the halves, and merges them back together. This implementation
/// uses Insertion Sort for small sub-arrays to improve performance.
///
/// # Performance
///
/// * **Time Complexity**: $O(n \log n)$ (worst/average/best case).
/// * **Space Complexity**: $O(n)$ (temporary buffers for merging).
/// * **Stability**: Stable.
pub fn merge_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    rec_merge_sort(array);
}

fn rec_merge_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let n: usize = array.len();
    
    // Optimization: Use insertion sort for small arrays (threshold 16-32)
    if n <= 16 {
        insertion_sort(array);
        return;
    }

    let mid: usize = n / 2;
    
    // Recursively sort the two halves
    rec_merge_sort(&mut array[..mid]);
    rec_merge_sort(&mut array[mid..]);

    // Merge the sorted halves
    merge(array, mid);
}

fn merge<T: PartialOrd + Clone>(array: &mut [T], mid: usize) {
    // Create temporary copies of the left and right halves
    let left: Vec<T> = array[..mid].to_vec();
    let right: Vec<T> = array[mid..].to_vec();

    let mut i: usize = 0; // index for left
    let mut j: usize = 0; // index for right
    let mut k: usize = 0; // index for original array

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            array[k] = left[i].clone();
            i += 1;
        } else {
            array[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Copy remaining elements
    if i < left.len() {
        array[k..].clone_from_slice(&left[i..]);
    }
    if j < right.len() {
        array[k..].clone_from_slice(&right[j..]);
    }
}
