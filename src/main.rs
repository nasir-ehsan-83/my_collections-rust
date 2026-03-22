use my_collections::sort::insertion_sort;

fn main() {
    let mut array: [i32; 7] = [3, 2, 6, 2, 8, 5, 2];
    println!("Before sorting: {:?}", &array);
    insertion_sort(&mut array);
    println!("After sorting: {:?}", &array);

    println!();

    let mut array1: [u32; 7] = [3, 2, 6, 2, 8, 5, 2];
    println!("Before sorting: {:?}", &array1);
    insertion_sort(&mut array1);
    println!("After soring: {:?}", &array1);

    println!();

    let mut character : [char; 5] = ['g', 'd', 'f', 'c', 'b'];

    println!("Before sorting: {:?}", character);
    insertion_sort(&mut character);
    println!("After sorting: {:?}", character);

    println!();

    let mut float: [f64; 5] = [34.6, 23.4, 76.0, 15.2, 28.6];
    println!("Before sorting: {:?}", float);
    insertion_sort(&mut float);
    println!("After sorting: {:?}", float);
}