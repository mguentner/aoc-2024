use std::collections::HashSet;

fn input_to_matrix(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn find_start_coords(matrix: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                result.push((y, x));
            }
        }
    }
    result
}

fn look_around(matrix: &Vec<Vec<u8>>, pos: (usize, usize)) -> Vec<(usize, usize, u8)> {
    let mut result = Vec::new();
    for (dy, dx) in [(-1i32, 0), (0, -1i32), (0, 1), (1, 0)] {
        if pos.0 as i32 + dy < 0 || pos.0 as i32 + dy >= matrix.len() as i32 {
            continue;
        }
        if pos.1 as i32 + dx < 0 || pos.1 as i32 + dx >= matrix[pos.0].len() as i32 {
            continue;
        }
        let y = (pos.0 as i32 + dy) as usize;
        let x = (pos.1 as i32 + dx) as usize;
        result.push((y, x, matrix[y][x]));
    }
    result
}

fn walk(matrix: &Vec<Vec<u8>>, pos: (usize, usize)) -> HashSet<(usize, usize)> {
    let current_node_value = matrix[pos.0][pos.1];
    if current_node_value == 9 {
        return HashSet::from([pos]);
    }
    let around =  look_around(matrix, pos).into_iter().filter(|&(_, _, value)| value == current_node_value + 1).collect::<Vec<_>>();
    let mut peaks = HashSet::new();
    for (idx, (y, x, _)) in around.iter().enumerate() {
        for peak in  walk(matrix, (*y,*x)) {
            peaks.insert(peak);
        }
    }
    peaks
}

fn walk2(matrix: &Vec<Vec<u8>>, pos: (usize, usize)) -> u32 {
    let current_node_value = matrix[pos.0][pos.1];
    if current_node_value == 9 {
        return 1;
    }
    let around =  look_around(matrix, pos).into_iter().filter(|&(_, _, value)| value == current_node_value + 1).collect::<Vec<_>>();
    let mut peaks = 0;
    for (idx, (y, x, _)) in around.iter().enumerate() {
        peaks += walk2(matrix,(*y,*x));
    }
    peaks
}

fn main() {
    let example = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    let input = include_str!("../input.txt");
    let example_matrix = input_to_matrix(example);
    let input_matrix = input_to_matrix(input);

    let mut example_sum = 0;
    for (y, x) in find_start_coords(&example_matrix) {
        let trail_sum = walk(&example_matrix, (y, x)).len();
        example_sum += trail_sum;
    }
    println!("Example Part 1: {}", example_sum);

    let mut example_sum = 0;
    for (y, x) in find_start_coords(&example_matrix) {
        let trail_sum = walk2(&example_matrix, (y, x));
        example_sum += trail_sum;
    }
    println!("Example Part 2: {}", example_sum);


    let mut input_sum = 0;
    for (y, x) in find_start_coords(&input_matrix) {
        let trail_sum = walk(&input_matrix, (y, x)).len();
        input_sum += trail_sum;
    }
    println!("Input Part 1: {}", input_sum);

    let mut input_sum = 0;
    for (y, x) in find_start_coords(&input_matrix) {
        let trail_sum = walk2(&input_matrix, (y, x));
        input_sum += trail_sum;
    }
    println!("Input Part 2: {}", input_sum);
}
