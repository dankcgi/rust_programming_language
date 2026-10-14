/*
 Given a list of integers, use a vector and return the median (when
 sorted, the value in the middle position) and mode (the value that
 occurs most often; a hash map will be helpful here) of the list.
*/
use std::collections::HashMap;

fn get_median(vector: &Vec<i32>) -> i32 {
    let mut copy = vector.clone();
    copy.sort();
    let length = copy.len();
    let half_way = length / 2;

    copy[half_way]
}

fn get_mode(vector: &Vec<i32>) -> i32 {
    let copy = vector.clone();
    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    for element in copy {
        let count = frequencies.entry(element).or_insert(0);
        *count += 1;
    }

    let mut count_vector: Vec<_> = frequencies.iter().collect();
    count_vector.sort_by(|a, b| b.1.cmp(a.1));

    *count_vector[0].0
}

pub fn exercise_1() {
    let integers = vec![10, 12, 9, 3, 10, 46, 200, 3, 150, 86, 10, 46, 69];

    let median = get_median(&integers);
    let mode = get_mode(&integers);
    println!("Exercise 1: get median and mode of integer vector");
    println!("Vector:\t{integers:?}");
    println!("Median:\t{median}");
    println!("Mode:\t{mode}");
    println!("");
}
