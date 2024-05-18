pub fn improved_bubble_sort(arr: &mut [usize]) -> &mut [usize] {
    let len = arr.len();
    let mut swapped = true;
   while swapped {
       swapped = false;
        for i in 1..len {
            if arr[i] < arr[i - 1] {
                arr.swap(i, i - 1);
                swapped = true;
            }
        }
    }
    arr
}
