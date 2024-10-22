use std::usize;

use rand::Rng;

struct Range<T> {
    min: T,
    max: T,
}

fn shuffle<T>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let rand_index = rand::thread_rng().gen_range(0..arr.len());
        arr.swap(i, rand_index);
    }
}

fn is_valid<T: std::cmp::PartialOrd>(arr: &[T]) -> bool {
    (1..arr.len())
        .map(|x| arr[x - 1] <= arr[x])
        .filter(|x| !*x)
        .count()
        == 0
}

fn bogosort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    while !is_valid(arr) {
        shuffle(arr);
    }
}

fn bubble_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut is_valid = true;
        for j in 0..arr.len() - i - 1 {
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

fn merge_sort<T: std::cmp::PartialOrd + Clone>(
    arr: &mut [T],
    mini: Option<usize>,
    maxi: Option<usize>,
) {
    let mini = mini.unwrap_or(0);
    let maxi = maxi.unwrap_or(arr.len() - 1);
    let length = maxi - mini + 1;

    // manually sort when lenght is lower or equal to two
    if length == 2 && arr[mini] > arr[maxi] {
        arr.swap(mini, maxi);
    }
    if length <= 2 {
        return;
    }

    // split the arr in 2 and sort those
    let middle = mini + (maxi - mini) / 2;
    merge_sort(arr, Some(mini), Some(middle));
    merge_sort(arr, Some(middle + 1), Some(maxi));

    // merge the 2 sorted arrays in sorted_arr
    let mut left_pointer = mini;
    let mut right_pointer = middle + 1;
    let mut sorted_arr: Vec<T> = Vec::new();
    for _ in mini..=maxi {
        if right_pointer > maxi || left_pointer <= middle && arr[left_pointer] <= arr[right_pointer]
        {
            sorted_arr.push(arr[left_pointer].clone());
            left_pointer += 1;
        } else {
            sorted_arr.push(arr[right_pointer].clone());
            right_pointer += 1;
        }
    }

    // sorted_arr -> arr
    for (i, value) in sorted_arr.iter().enumerate() {
        arr[mini + i] = value.clone();
    }
    return;
}

fn hash_sort(arr: &mut [i32], range: Range<i32>) {
    assert!(range.max > range.min);
    let range_length = (range.max - range.min).abs() as usize;
    let mut hashtable = vec![0; range_length];
    for num in &mut *arr {
        let index = (*num - range.min) as usize;
        hashtable[index] += 1;
    }

    let mut index = 0;
    for i in 0..range_length {
        for _ in 0..hashtable[i] {
            assert!(index != arr.len());
            arr[index] = i as i32 + range.min;
            index += 1;
        }
    }
}

fn main() {
    const LENGTH: usize = 8;

    let mut arr: [usize; LENGTH] = [0; LENGTH].map(|_| rand::thread_rng().gen_range(0..100));
    println!("bubble sort");
    println!("{:?}", arr);
    bubble_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr: [usize; LENGTH] = [0; LENGTH].map(|_| rand::thread_rng().gen_range(0..100));
    println!("merge sort");
    println!("{:?}", arr);
    merge_sort(&mut arr, None, None);
    println!("{:?}", arr);

    let mut arr: [i32; LENGTH] = [0; LENGTH].map(|_| rand::thread_rng().gen_range(-100..100));
    println!("hash sort");
    println!("{:?}", arr);
    hash_sort(
        &mut arr,
        Range {
            min: -100,
            max: 100,
        },
    );
    println!("{:?}", arr);

    let mut arr: [usize; LENGTH] = [0; LENGTH].map(|_| rand::thread_rng().gen_range(0..100));
    println!("bogosort");
    println!("{:?}", arr);
    bogosort(&mut arr);
    println!("{:?}", arr);

    let mut arr: [char; 4] = ['1', '2', '4', '3'];
    println!("bogosort");
    println!("{:?}", arr);
    bogosort(&mut arr);
    println!("{:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn randomize_array(range: Range<i64>, array: &mut [i64]) -> &mut [i64] {
        let mut rng = rand::thread_rng();
        for i in 0..array.len() {
            let random_value = rng.gen_range(range.min..range.max);
            array[i] = random_value;
        }
        return array;
    }

    fn randomize_array_i32(range: Range<i32>, array: &mut [i32]) -> &mut [i32] {
        let mut rng = rand::thread_rng();
        for i in 0..array.len() {
            let random_value = rng.gen_range(range.min..range.max);
            array[i] = random_value;
        }
        return array;
    }

    #[test]
    fn is_valid1() {
        let input = [1, 2, 3];
        assert!(is_valid(&input));
    }
    #[test]
    fn is_valid2() {
        let input = [1, 3, 2];
        assert!(!is_valid(&input));
    }

    #[test]
    fn bogosort1() {
        let mut input = [0; 6];
        let input = randomize_array(Range { min: 0, max: 1000 }, &mut input);
        bogosort(input);
        assert!(is_valid(input));
    }
    #[test]
    fn bogosort2() {
        let mut input = [0; 6];
        let input = randomize_array(
            Range {
                min: -1000,
                max: 1000,
            },
            &mut input,
        );
        bogosort(input);
        assert!(is_valid(input));
    }
    #[test]
    fn bogosort3() {
        let mut input = [0; 6];
        let input = randomize_array(Range { min: -1000, max: 0 }, &mut input);
        bogosort(input);
        assert!(is_valid(input));
    }

    #[test]
    fn bubble_sort1() {
        let mut input = [0; 1000];
        let input = randomize_array(Range { min: 0, max: 1000 }, &mut input);
        bubble_sort(input);
        assert!(is_valid(input));
    }
    #[test]
    fn bubble_sort2() {
        let mut input = [0; 1000];
        let input = randomize_array(
            Range {
                min: -1000,
                max: 1000,
            },
            &mut input,
        );
        bubble_sort(input);
        assert!(is_valid(input));
    }
    #[test]
    fn bubble_sort3() {
        let mut input = [0; 1000];
        let input = randomize_array(Range { min: -1000, max: 0 }, &mut input);
        bubble_sort(input);
        assert!(is_valid(input));
    }

    #[test]
    fn merge_sort1() {
        let mut input = [0; 1000];
        let input = randomize_array(Range { min: 0, max: 1000 }, &mut input);
        merge_sort(input, None, None);
        assert!(is_valid(input));
    }
    #[test]
    fn merge_sort2() {
        let mut input = [0; 1000];
        let input = randomize_array(
            Range {
                min: -1000,
                max: 1000,
            },
            &mut input,
        );
        merge_sort(input, None, None);
        assert!(is_valid(input));
    }
    #[test]
    fn merge_sort3() {
        let mut input = [0; 1000];
        let input = randomize_array(Range { min: -1000, max: 0 }, &mut input);
        merge_sort(input, None, None);
        assert!(is_valid(input));
    }

    #[test]
    fn hash_sort1() {
        let mut input = [0; 1000];
        let input = randomize_array_i32(Range { min: 0, max: 1000 }, &mut input);
        hash_sort(input, Range { min: 0, max: 1000 });
        assert!(is_valid(input));
    }
    #[test]
    fn hash_sort2() {
        let mut input = [0; 1000];
        let input = randomize_array_i32(Range { min: -1000, max: 0 }, &mut input);
        hash_sort(input, Range { min: -1000, max: 0 });
        assert!(is_valid(input));
    }
    #[test]
    fn hash_sort3() {
        let mut input = [0; 2000];
        let input = randomize_array_i32(
            Range {
                min: -1000,
                max: 1000,
            },
            &mut input,
        );
        hash_sort(
            input,
            Range {
                min: -1000,
                max: 1000,
            },
        );
        assert!(is_valid(input));
    }
}
