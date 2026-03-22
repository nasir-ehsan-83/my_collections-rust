/// Sorts a array in-place using the insertion sort algorithm.
///
/// Insertion sort is $O(n^2)$ in the average and worst cases, but $O(n)$ 
/// for nearly-sorted data. It is efficient for very small datasets.
///
/// # Examples
///
/// ```
/// let mut numbers = [4, 2, 5, 1, 3];
/// my_collections::sort::insertion_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```

pub fn insertion_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    // 1. We start at index 1 because the first element is "sorted" by itself
    for i in 1..array.len() {
        // 2. "Pick up" the current element (this is the Key)
        let key = array[i].clone();
        let mut j = i;

        // 3. Shift elements to the right as long as they are larger than the key
        // We use 'j > 0' to prevent going out of bounds
        while j > 0 && array[j - 1] > key {
            array[j] = array[j - 1].clone(); // Shift the larger element right
            j -= 1; // Move the "hole" to the left
        }

        // 4. Drop the key into the final "hole"
        array[j] = key;
    }
}
