pub fn merge_sort(arr: &mut [usize], begin: usize, end: usize) -> &mut [usize] {
    if begin < end {
        let mid = begin + (end - begin) / 2;

        merge_sort(arr, begin, mid);
        merge_sort(arr, mid + 1, end);
        merge(arr, begin, mid, end);
    }
    arr
}

fn merge(arr: &mut [usize], left: usize, mid: usize, right: usize) {

    let sub_one = mid - left + 1;
    let sub_two = right - mid;

    let mut left_array = vec![0; sub_one];
    let mut right_array = vec![0; sub_two];

    for i in 0..sub_one {
        left_array[i] = arr[left + i];
    }
    for j in 0..sub_two {
        right_array[j] = arr[mid + 1 + j];
    }

    let mut index_sub_one = 0;
    let mut index_sub_two = 0;
    let mut index_merged_arr = left;

    while index_sub_one < sub_one && index_sub_two < sub_two {
        if left_array[index_sub_one] <= right_array[index_sub_two] {
            arr[index_merged_arr] = left_array[index_sub_one];
            index_sub_one += 1;
        } else {
            arr[index_merged_arr] = right_array[index_sub_two];
            index_sub_two += 1;
        }
        index_merged_arr += 1;
    }

    while index_sub_one < sub_one {
        arr[index_merged_arr] = left_array[index_sub_one];
        index_sub_one += 1;
        index_merged_arr += 1;
    }

    while index_sub_two < sub_two {
        arr[index_merged_arr] = right_array[index_sub_two];
        index_sub_two += 1;
        index_merged_arr += 1;
    }
}

// let mut index = String::new();
//
// io::stdin()
// .read_line(&mut index)
// .expect("Failed to read line");
//
// let index: usize = index
// .trim()
// .parse()
// .expect("Index entered was not a number");

