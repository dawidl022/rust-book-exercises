use std::collections::HashMap;

fn main() {
    assert_eq!(3, median(vec![2, 1, 4, 7]));
    assert_eq!(2, median(vec![2, 1, 8]));

    assert_eq!(7, mode(vec![2, 7, 1, 4, 7]));
}

fn median(numbers: Vec<i32>) -> i32 {
    let mut numbers = numbers.clone();
    numbers.sort();

    let mid = numbers.len() / 2;

    if numbers.len() % 2 == 0 {
        let mid2 = mid - 1;
        (numbers[mid] + numbers[mid2]) / 2
    } else {
        numbers[mid]
    }
}

fn mode(numbers: Vec<i32>) -> i32 {
    let mut frequencies: HashMap<i32, u32> = HashMap::new();

    for num in &numbers {
        let freq = frequencies.entry(*num).or_insert(0);
        *freq += 1;
    }

    let mut current_mode = numbers[0];
    let mut highest_frequency = frequencies[&numbers[0]];

    for (num, freq) in frequencies {
        if freq > highest_frequency {
            current_mode = num;
            highest_frequency = freq;
        }
    }

    current_mode
}
