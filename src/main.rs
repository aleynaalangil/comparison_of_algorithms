use assignment_311::bubble_sort::bubble_sort;
use assignment_311::quick_sort::quick_sort;
use rand::Rng;
use assignment_311::improved_bubble_sort::improved_bubble_sort;
use assignment_311::improved_quick_sort::improved_quick_sort;
use assignment_311::merge_sort::merge_sort;
use assignment_311::selection_sort::selection_sort;

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr: [usize; 10] = [0; 10]; // Initialize an array of length 10

    for i in 0..10 {
        arr[i] = rng.gen_range(0..100); // Fill the array with random u32 values
    }
    println!("array: {:?}", arr);
    let n = arr.len();

    // let result = bubble_sort(&mut arr);
    // let result = quick_sort(&mut arr, 0, (n - 1).try_into().unwrap());
    // let result = merge_sort(&mut arr, 0, (n - 1).try_into().unwrap());
    // let result = selection_sort(&mut arr, n);
    // let result = improved_bubble_sort(&mut arr);
    let result = improved_quick_sort(&mut arr, 0, (n - 1).try_into().unwrap());

    println!("sorted: {:?}", result);
}
