use assignment_311::bubble_sort::bubble_sort;
use assignment_311::quick_sort::quick_sort;
use assignment_311::improved_bubble_sort::improved_bubble_sort;
use assignment_311::improved_quick_sort::improved_quick_sort;
use assignment_311::merge_sort::merge_sort;
use assignment_311::selection_sort::selection_sort;
use std::time::{Instant, Duration};
use rand::Rng;

macro_rules! clone_array {
    ($arr:expr, $($name:ident),+) => {
        $(let mut $name = $arr.clone();)+
    };
}

fn generate_random_array(rng: &mut rand::rngs::ThreadRng, size: usize) -> Vec<usize> {
    let mut arr = Vec::with_capacity(size);
    for _ in 0..size {
        arr.push(rng.gen_range(0..100));
    }
    arr
}

macro_rules! measure_time_taken_individual {
    ($algorithm:ident, $arr:ident, $name:expr) => {
        let start = Instant::now();
        $algorithm(&mut $arr);
        let duration = start.elapsed();
        println!("{} took: {:?}", $name, duration)
    };
    ($algorithm:ident, $arr:ident, $name:expr, $($args:expr),*) =>{
       let start = Instant::now();
        $algorithm(&mut $arr, $($args), *);
        let duration = start.elapsed();
        println!("{} took: {:?}", $name, duration)
    }
}

macro_rules! measure_time_taken {
    ($sort_fn:ident, $arr:ident, $name:expr $(, $args:expr)*) => {{
        let mut total_duration = Duration::new(0, 0);
        const NUM_RUNS: usize = 5; // Adjust as needed
        for _ in 0..NUM_RUNS {
            let start = Instant::now();
            $sort_fn(&mut $arr $(, $args)*);
            let duration = start.elapsed();
            total_duration += duration;
        }
        let avg_duration = total_duration / NUM_RUNS as u32;
        println!("{} took: {:?}", $name, avg_duration);
    }};
}

fn main() {
    let mut rng = rand::thread_rng();
    let arr_10 = generate_random_array(&mut rng, 10);
    let arr_100 = generate_random_array(&mut rng, 100);
    let arr_1000 = generate_random_array(&mut rng, 1000);
    let arr_10000 = generate_random_array(&mut rng, 10000);
    let arr_100000 = generate_random_array(&mut rng, 100000);

    clone_array!(arr_10, arr_bubble_10, arr_quick_10, arr_merge_10, arr_selection_10, arr_improved_bubble_10, arr_improved_quick_10);
    clone_array!(arr_100, arr_bubble_100, arr_quick_100, arr_merge_100, arr_selection_100, arr_improved_bubble_100, arr_improved_quick_100);
    clone_array!(arr_1000, arr_bubble_1000, arr_quick_1000, arr_merge_1000, arr_selection_1000, arr_improved_bubble_1000, arr_improved_quick_1000);
    clone_array!(arr_10000, arr_bubble_10000, arr_quick_10000, arr_merge_10000, arr_selection_10000, arr_improved_bubble_10000, arr_improved_quick_10000);
    clone_array!(arr_100000, arr_bubble_100000, arr_quick_100000, arr_merge_100000, arr_selection_100000, arr_improved_bubble_100000, arr_improved_quick_100000);

    // Ignore the first run
    bubble_sort(&mut arr_bubble_10);
    bubble_sort(&mut arr_bubble_100);
    bubble_sort(&mut arr_bubble_1000);
    bubble_sort(&mut arr_bubble_10000);
    bubble_sort(&mut arr_bubble_100000);

    measure_time_taken!(bubble_sort, arr_bubble_10, "Bubble sort");
    measure_time_taken!(bubble_sort, arr_bubble_100, "Bubble sort");
    measure_time_taken!(bubble_sort, arr_bubble_1000, "Bubble sort");
    measure_time_taken!(bubble_sort, arr_bubble_10000, "Bubble sort");
    measure_time_taken!(bubble_sort, arr_bubble_100000, "Bubble sort");

    // Ignore the first run
    improved_bubble_sort(&mut arr_improved_bubble_10);
    improved_bubble_sort(&mut arr_improved_bubble_100);
    improved_bubble_sort(&mut arr_improved_bubble_1000);
    improved_bubble_sort(&mut arr_improved_bubble_10000);
    improved_bubble_sort(&mut arr_improved_bubble_100000);

    measure_time_taken!(improved_bubble_sort, arr_improved_bubble_10, "Improved bubble sort");
    measure_time_taken!(improved_bubble_sort, arr_improved_bubble_100, "Improved bubble sort");
    measure_time_taken!(improved_bubble_sort, arr_improved_bubble_1000, "Improved bubble sort");
    measure_time_taken!(improved_bubble_sort, arr_improved_bubble_10000, "Improved bubble sort");
    measure_time_taken!(improved_bubble_sort, arr_improved_bubble_100000, "Improved bubble sort");

    // Ignore the first run
    quick_sort(&mut arr_quick_10, 0, 9);
    quick_sort(&mut arr_quick_1000, 0, 99);
    quick_sort(&mut arr_quick_1000, 0, 999);
    quick_sort(&mut arr_quick_100000,0, 9999);
    quick_sort(&mut arr_quick_100000, 0, 99999);

    measure_time_taken!(quick_sort, arr_quick_10, "Quick sort", 0, 9);
    measure_time_taken!(quick_sort, arr_quick_100, "Quick sort", 0, 99);
    measure_time_taken!(quick_sort, arr_quick_1000, "Quick sort", 0, 999);
    measure_time_taken!(quick_sort, arr_quick_10000, "Quick sort", 0, 9999);
    measure_time_taken!(quick_sort, arr_quick_100000, "Quick sort", 0, 99999);

    // Ignore the first run
    improved_quick_sort(&mut arr_improved_quick_10, 0, 9);
    improved_quick_sort(&mut arr_improved_quick_1000, 0, 99);
    improved_quick_sort(&mut arr_improved_quick_1000, 0, 999);
    improved_quick_sort(&mut arr_improved_quick_100000,0, 9999);
    improved_quick_sort(&mut arr_improved_quick_100000, 0, 99999);

    measure_time_taken!(improved_quick_sort, arr_improved_quick_10, "Improved quick sort", 0, 9);
    measure_time_taken!(improved_quick_sort, arr_improved_quick_100, "Improved quick sort", 0, 99);
    measure_time_taken!(improved_quick_sort, arr_improved_quick_1000, "Improved quick sort", 0, 999);
    measure_time_taken!(improved_quick_sort, arr_improved_quick_10000, "Improved quick sort", 0, 9999);
    measure_time_taken!(improved_quick_sort, arr_improved_quick_100000, "Improved quick sort", 0, 99999);

    // Ignore the first run
    merge_sort(&mut arr_merge_10, 0, (10 - 1));
    merge_sort(&mut arr_merge_100, 0, (100 - 1));
    merge_sort(&mut arr_merge_1000, 0, (1000 - 1));
    merge_sort(&mut arr_merge_10000,0, (10000 - 1));
    merge_sort(&mut arr_merge_100000, 0, (100000 - 1));

    measure_time_taken!(merge_sort, arr_merge_10, "Merge sort", 0, (10 - 1));
    measure_time_taken!(merge_sort, arr_merge_100, "Merge sort", 0, (100 - 1));
    measure_time_taken!(merge_sort, arr_merge_1000, "Merge sort", 0, (1000 - 1));
    measure_time_taken!(merge_sort, arr_merge_10000, "Merge sort", 0, (10000 - 1));
    measure_time_taken!(merge_sort, arr_merge_100000, "Merge sort", 0, (100000 - 1));

    // Ignore the first run
    selection_sort(&mut arr_selection_10, 10);
    selection_sort(&mut arr_selection_100, 100);
    selection_sort(&mut arr_selection_1000, 1000);
    selection_sort(&mut arr_selection_10000,10000);
    selection_sort(&mut arr_selection_100000, 100000);

    measure_time_taken!(selection_sort, arr_selection_10, "Selection sort", 10);
    measure_time_taken!(selection_sort, arr_selection_100, "Selection sort", 100);
    measure_time_taken!(selection_sort, arr_selection_1000, "Selection sort", 1000);
    measure_time_taken!(selection_sort, arr_selection_10000, "Selection sort", 10000);
    measure_time_taken!(selection_sort, arr_selection_100000, "Selection sort", 100000);

}