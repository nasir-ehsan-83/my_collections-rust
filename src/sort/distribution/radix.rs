/// Sorts a slice of non-negative integers using the radix sort algorithm (LSD).
///
/// Radix sort avoids comparison by creating and distributing elements into buckets 
/// according to their radix. This version uses Least Significant Digit (LSD).
///
/// # Performance
///
/// * **Time Complexity**: $O(d \cdot (n + k))$, where $d$ is number of digits and $k$ is the radix (base).
/// * **Space Complexity**: $O(n + k)$.
/// * **Stability**: This implementation is stable.
///
/// # Examples
///
/// ```
/// use my_collections::sort::radix_sort;
///
/// let mut numbers = [170, 45, 75, 90, 802, 24, 2, 66];
/// radix_sort(&mut numbers);
/// assert_eq!(numbers, [2, 24, 45, 66, 75, 90, 170, 802]);
/// ```
pub fn radix_sort(array: &mut [usize]) {
    if array.is_empty() {
        return;
    }

    let max = *array.iter().max().unwrap();
    let mut exp = 1;

    while max / exp > 0 {
        counting_sort_by_digit(array, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(array: &mut [usize], exp: usize) {
    let mut output = vec![0; array.len()];
    let mut count = vec![0; 10];

    for &num in array.iter() {
        count[(num / exp) % 10] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in array.iter().rev() {
        let digit = (num / exp) % 10;
        output[count[digit] - 1] = num;
        count[digit] -= 1;
    }

    array.copy_from_slice(&output);
}