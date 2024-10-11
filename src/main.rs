use rand::Rng;

fn shuffle(arr: &mut [u8], length: usize) {
    for i in 0..length {
        let temp = arr[i];
        let rand_index = rand::thread_rng().gen_range(0..10);
        arr[i] = arr[rand_index];
        arr[rand_index] = temp;
    }
}

fn bogosort(arr: &mut [u8], length: usize) {
    loop {
        let mut is_valid = true;
        for i in 1..length {
            if arr[i - 1] > arr[i] {
                is_valid = false;
                break;
            }
            if i == length - 1 {
                break;
            }
        }
        if is_valid {
            return;
        }
        shuffle(arr, length);
    }
}

fn main() {
    let mut arr: [u8; 10] = [0; 10].map(|_| rand::thread_rng().gen_range(0..100));
    println!("{:?}", arr);
    bogosort(&mut arr, 10);
    println!("{:?}", arr);
}
