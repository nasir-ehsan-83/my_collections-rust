use my_collections::sort::{
    insertion_sort,
    bubble_sort,
    selection_sort
};

fn main() {
    
    let mut array_i: [i32; 7] = [3, 2, 6, 2, 8, 5, 2];
    println!("Insertoin sort ");
    println!("Before sorting: {:?}", &array_i);
    insertion_sort(&mut array_i);
    println!("After sorting: {:?}", &array_i);

    println!();
    /*
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

    */
    println!("Bubble sort");
    let mut array_b: [i32; 7] = [3, 2, 6, 2, 8, 5, 2];
    println!("Before sorting: {:?}", &array_b);
    bubble_sort(&mut array_b);
    println!("After sorting: {:?}", &array_b);

    println!();

    println!("Selection sort");
    let mut array_s: [i32; 7] = [3, 6, 2, 6, 2, 7, 1];
    println!("Before sorting: {:?}", &array_s);
    selection_sort(&mut array_s);
    println!("After soring: {:?}", &array_s);
}