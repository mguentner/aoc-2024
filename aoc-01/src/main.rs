use std::iter::zip;
use std::str;

fn find_distance(list_a: &[i32], list_b: &[i32]) -> i32 {
    let mut list_a_sorted = Vec::from(list_a);
    let mut list_b_sorted = Vec::from(list_b);
    list_a_sorted.sort();
    list_b_sorted.sort();
    zip(list_a_sorted.iter(), list_b_sorted.iter()).map(|x| (x.0 - x.1).abs()).sum()
}

fn calculate_similarity(list_a: &[i32], list_b: &[i32]) -> i32 {
    let mut count = 0;
    for x in list_a {
        count += x*list_b.iter().filter(|y| *y == x).count() as i32;
    }
    count
}

fn get_lists(data: &[u8]) -> (Vec<i32>, Vec<i32>) {
    let as_str = str::from_utf8(data).unwrap();
    as_str.lines().map(|line| line.split_once(" ")).flatten().map(|(first, second)| (first.trim().parse::<i32>().unwrap(), second.trim().parse::<i32>().unwrap())).collect()
}

fn main() {
    let example_data = include_bytes!("../example.txt");
    let input_data = include_bytes!("../input.txt");
    let (example_list_a, example_list_b) = get_lists(example_data);
    let dist_example = find_distance(&example_list_a, &example_list_b);
    let (input_list_a, input_list_b) = get_lists(input_data);
    let dist_input = find_distance(&input_list_a, &input_list_b);
    println!("Example Distance {:?}", dist_example);
    println!("Input Distance {:?}", dist_input);
    
    let example_similarity = calculate_similarity(&example_list_a, &example_list_b);
    println!("Example Similarity: {:?}", example_similarity);
    
    let input_similarity = calculate_similarity(&input_list_a, &input_list_b);
    println!("Input Similarity: {:?}", input_similarity)
}
