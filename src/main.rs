use std::usize;

use rand::Rng;

fn shuffle(arr: &mut [usize], length: usize) {
    for i in 0..length {
        let rand_index = rand::thread_rng().gen_range(0..length);
        arr.swap(i, rand_index);
    }
}

fn is_valid(arr: &[usize], length: usize) -> bool {
    (1..length)
        .map(|x| arr[x - 1] <= arr[x])
        .filter(|x| !*x)
        .count()
        == 0
}

fn bogosort(arr: &mut [usize], length: usize) {
    while !is_valid(arr, length) {
        shuffle(arr, length);
    }
}

fn bubble_sort(arr: &mut [usize], length: usize) {
    for i in 0..length - 1 {
        let mut is_valid = true;
        for j in 0..length - i - 1 {
            if arr[j] > arr[j + 1] {
                is_valid = false;
                arr.swap(j, j + 1);
            }
        }
        if is_valid {
            break;
        }
    }
}

fn merge_sort(arr: &mut [usize], length: usize, mini: Option<usize>, maxi: Option<usize>) {
    let mini = mini.unwrap_or(0);
    let maxi = maxi.unwrap_or(length - 1);
    assert!(maxi - mini == length - 1);

    // manually sort when lenght is lower or equal to two
    if length == 2 && arr[mini] > arr[maxi] {
        arr.swap(mini, maxi);
    }
    if length <= 2 {
        return;
    }

    // split the arr in 2 and sort those
    let middle = mini + (maxi - mini) / 2;
    merge_sort(arr, length / 2 + length % 2, Some(mini), Some(middle));
    merge_sort(arr, length / 2, Some(middle + 1), Some(maxi));

    // merge the 2 sorted arrays in sorted_arr
    let mut left_pointer = mini;
    let mut right_pointer = middle + 1;
    let mut sorted_arr: Vec<usize> = Vec::new();
    for _ in mini..=maxi {
        if right_pointer > maxi || left_pointer <= middle && arr[left_pointer] <= arr[right_pointer]
        {
            sorted_arr.push(arr[left_pointer]);
            left_pointer += 1;
        } else {
            sorted_arr.push(arr[right_pointer]);
            right_pointer += 1;
        }
    }

    // sorted_arr -> arr
    for (i, value) in sorted_arr.iter().enumerate() {
        arr[mini + i] = *value;
    }
    return;
}

fn hash_sort(arr: &mut [usize], length: usize, range: (usize, usize)) {
    assert!(range.1 > range.0);
    let mut hashtable = vec![0; range.1 - range.0];
    for num in &mut *arr {
        hashtable[*num] += 1;
    }

    let mut index = 0;
    for i in range.0..range.1 {
        for _ in 0..hashtable[i] {
            assert!(index != length);
            arr[index] = i;
            index += 1;
        }
    }
}

fn main() {
    const LENGTH: usize = 8;

    let mut arr: [usize; LENGTH] = [0; LENGTH].map(|_| rand::thread_rng().gen_range(0..100));
    println!("bubble sort");
    println!("{:?}", arr);
    bubble_sort(&mut arr, LENGTH);
    println!("{:?}", arr);

    let mut arr: [usize; LENGTH] = [0; LENGTH].map(|_| rand::thread_rng().gen_range(0..100));
    println!("merge sort");
    println!("{:?}", arr);
    merge_sort(&mut arr, LENGTH, None, None);
    println!("{:?}", arr);

    let mut arr: [usize; LENGTH] = [0; LENGTH].map(|_| rand::thread_rng().gen_range(0..100));
    println!("hash sort");
    println!("{:?}", arr);
    hash_sort(&mut arr, LENGTH, (0, 100));
    println!("{:?}", arr);

    let mut arr: [usize; LENGTH] = [0; LENGTH].map(|_| rand::thread_rng().gen_range(0..100));
    println!("bogosort");
    println!("{:?}", arr);
    bogosort(&mut arr, LENGTH);
    println!("{:?}", arr);
}
