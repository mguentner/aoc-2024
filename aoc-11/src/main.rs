use std::collections::HashMap;

fn input_to_vector(input: &str) -> Vec<u64> {
    input
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn blink_stone(value: u64) -> Vec<u64> {
    if value == 0 {
        vec![1]
    } else {
        let as_str = value.to_string();
        if as_str.chars().count() % 2 == 0 {
            let middle = as_str.chars().count() / 2;
            let split = as_str.split_at(middle);
            let first = split.0.parse::<u64>().unwrap();
            let second = split.1.parse::<u64>().unwrap();
            vec![first, second]
        } else {
            vec![value * 2024]
        }
    }
}

fn blink(state: Vec<u64>, times: u64) -> usize {
    if times == 0 {
        return state.len();
    }
    let mut res = 0;
    for value in state.into_iter() {
        let stone_res = blink_stone(value);
        res += blink(stone_res, times - 1);
    }
    res
}

fn main() {
    let example = "125 17";
    let input = "8069 87014 98 809367 525 0 9494914 5";
    let example_vec = input_to_vector(example);

    let sum = example_vec
        .iter()
        .map(|x| blink(vec![*x], 25))
        .sum::<usize>();
    println!("{:?}", sum);

    let input_vec = input_to_vector(input);
    let sum = input_vec.iter().map(|x| blink(vec![*x], 25)).sum::<usize>();
    println!("Part 1: {:?}", sum);
    // Too complex for part 2 ^

    let mut counts: HashMap<u64, usize> = HashMap::new();
    for x in input_vec {
        counts.entry(x).and_modify(|f| *f += 1).or_insert(1);
    }
    for i in 0..75 {
        let mut new_counts: HashMap<u64, usize> = HashMap::new();
        for (k, v) in counts.iter() {
            new_counts.insert(*k, *v);
        }
        for (stone, count) in counts.iter() {
            let res = blink_stone(*stone);
            for v in res {
                new_counts
                    .entry(v)
                    .and_modify(|f| *f += count)
                    .or_insert(*count);
            }
            new_counts.entry(*stone).and_modify(|f| *f -= count);
        }
        counts.clear();
        for (k, v) in new_counts.iter() {
            if *v != 0 {
                counts.insert(*k, *v);
            }
        }
    }
    println!("Part 2: {:?}", counts.values().sum::<usize>());
}
