/// A trait for types that can be used in counting sort.
///
/// Types implementing this trait must be convertible to and from a non-negative index.
pub trait CountingSortable: Copy {
    /// Converts the value to an index.
    fn to_index(&self) -> usize;

    /// Creates a value from an index.
    fn from_index(index: usize) -> Self;
}

impl CountingSortable for usize {
    fn to_index(&self) -> usize {
        *self
    }

    fn from_index(index: usize) -> Self {
        index
    }
}

/// Sorts a slice in-place using the counting sort algorithm.
///
/// Counting sort works by counting occurrences of each value and reconstructing
/// the array using those counts.
///
/// This implementation is generic over types that implement `CountingSortable`.
///
/// # Performance
///
/// * **Time Complexity**: O(n + k)
/// * **Space Complexity**: O(k)
/// * **Stability**: Unstable (can be made stable with extra memory)
///
/// where:
/// - n = number of elements
/// - k = max value range
///
/// # Examples
///
/// ```
/// let mut numbers = [4usize, 2, 2, 8, 3, 3, 1];
/// counting_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 2, 3, 3, 4, 8]);
/// ```
pub fn counting_sort<T: CountingSortable>(array: &mut [T]) {
    if array.len() < 2 {
        return;
    }

    let max = array.iter().map(|x| x.to_index()).max().unwrap();
    let mut count = vec![0; max + 1];

    // Count occurrences
    for &item in array.iter() {
        count[item.to_index()] += 1;
    }

    // Reconstruct array
    let mut index = 0;
    for value in 0..=max {
        for _ in 0..count[value] {
            array[index] = T::from_index(value);
            index += 1;
        }
    }
}