mod binary_search;
mod bogo_sort;
mod bubble_sort;
mod string_reverser;
// mod linked_list;
// mod reverse_linked_list;

use crate::binary_search::binary_search;
use crate::bogo_sort::bogo_sort;
use crate::bubble_sort::bubble_sort;
use crate::string_reverser::reverse_string;

// use crate::linked_list::LinkedList;
// use crate::reverse_linked_list::reverse;

fn main() {
    //----------------------------- Reverse Linked List ----------------------------- //
    // LinkedList list = new LinkedList();

    // ------------------------------------ O(n) ------------------------------------ //
    let mut sorted_array: [i32; 10] = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91];

    let target = 2;
    match binary_search(&sorted_array, target) {
        Some(index) => println!("Found {target} at index {index}"),
        None => println!("{target} not found in the array"),
    }

    let target_not_found = 15;
    match binary_search(&sorted_array, target_not_found) {
        Some(index) => println!("Found {target_not_found} at index {index}"),
        None => println!("{target_not_found} not found in the array"),
    }

    // -------------------------------- O(n ^ 2) -------------------------------- //
    let mut my_array = [24, 34, 11, 35, 68, 90, 75, 111, 67, 150];
    bubble_sort(&mut my_array);
    println!("Sorted array: {my_array:?}");

    //----------------------------- O(n!) ----------------------------- //
    bogo_sort(&mut sorted_array);
    println!("Sorted array: {:?}", sorted_array);

    //----------------------------- String Reverser med Stack ----------------------------- //
    let original = "Hello, Rust!";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}

// fn big_o_testing() {
//     let random_array: [i32; 5] = [1, 2, 3, 4, 5];

//     // O(1) Konstant
//     let element = random_array[2];
//     println!("Element at index 2: {element}");

//     // --------------------------------------------------

//     let mut random_vector = vec![1, 2, 3, 4, 5];

//     // O(1)
//     let element = random_vector[2];
//     println!("Element at index 2: {element}");

//     // O(1)
//     random_vector.push(6);
//     println!("Vector after push: {random_vector:?}");

//     // O(n) -
//     random_vector.insert(0, 0);
//     println!("Vector after insert: {random_vector:?}");

//     let my_array = [1, 2, 3, 4, 5];
//     println!("Printing array:");
//     print_array(&my_array);

//     let my_vector = vec![10, 20, 30, 40, 50, 60];
//     println!("\nPrinting vector:");
//     print_array(&my_vector);
// }

// fn print_array(arr: &[i32]) {
//     for element in arr.iter() {
//         println!("{element}");
//     }
// }
