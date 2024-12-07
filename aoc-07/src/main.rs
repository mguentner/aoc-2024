use std::{io, iter};
use itertools::Itertools;

#[derive(Debug)]
struct Line {
    target: u64,
    values: Vec<u64>
}

impl TryFrom<&str> for Line {
    type Error = io::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split(": ");

        let target = parts
           .next()
           .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "no target") )?
           .parse::<u64>()
           .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        let values = parts
           .next()
           .ok_or( io::Error::new(io::ErrorKind::InvalidInput, "no values"))?
           .split_whitespace()
           .flat_map(|s| s.parse())
           .collect::<Vec<u64>>();

        Ok(Line {
            target,
            values
        })
    }
}

fn parse_to_lines(s: &str) -> Vec<Line> {
    s.lines()
       .flat_map(|l| l.try_into())
       .collect::<Vec<_>>()
}

#[derive(Debug)]
enum OperatorPart1 {
    Add,
    Mul
}

#[derive(Clone, Copy, Debug)]
enum OperatorPart2 {
    Add,
    Mul,
    Concat
}

fn generate_operator_permutation_part1(length: u32) -> Vec<Vec<OperatorPart1>> {
    let two: u32 = 2;
    let mut result = Vec::new();
    for i in 0..two.pow(length) {
        let mut permutation = Vec::new();
        for bit in (0..length).rev() {
            if i & two.pow(bit) == 0 {
                permutation.push(OperatorPart1::Add)
            } else {
                permutation.push(OperatorPart1::Mul)
            }
        }
        result.push(permutation);
    }
    result      
}

fn generate_operator_permutation_part2(length: u32) -> Vec<Vec<OperatorPart2>>  {
    let operators = vec![OperatorPart2::Add, OperatorPart2::Mul, OperatorPart2::Concat];
    iter::repeat(operators).take(length as usize).multi_cartesian_product().collect::<Vec<_>>()
}


fn main() {
    let example = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    let lines = parse_to_lines(example);
    println!("line {:?}", lines);
    
    let input = include_str!("../input.txt");
    let input_lines  = parse_to_lines(input);
    
    let sum = find_valids_part1(&lines);
    println!("example sum1 {}", sum);
    let sum = find_valids_part1(&input_lines);
    println!("input sum1 {}", sum);
    
    let permutations = generate_operator_permutation_part2(5);
    println!("permutations {:?}", permutations);
        
    let sum = find_valids_part2(&lines);
    println!("example sum2 {}", sum);
    let sum = find_valids_part2(&input_lines);
    println!("input sum2 {}", sum);
}

fn find_valids_part1(lines: &Vec<Line>) -> u64 {
    let mut sum = 0;
    for line in lines {
        for permutation in generate_operator_permutation_part1(line.values.len() as u32 - 1) {
            let mut result = line.values[0];
            for (i, op) in permutation.iter().enumerate() {
                match *op {
                    OperatorPart1::Add  => result += line.values[i + 1],
                    OperatorPart1::Mul  => result *= line.values[i + 1]
                }
            }
            if result == line.target {
                sum += line.target;
                break;
            }
        }
    }
    sum
}

fn find_valids_part2(lines: &Vec<Line>) -> u64 {
    let mut sum = 0;
    for line in lines {
        for permutation in generate_operator_permutation_part2(line.values.len() as u32 - 1) {
            let mut result = line.values[0];
            for (i, op) in permutation.iter().enumerate() {
                match *op {
                    OperatorPart2::Add  => result += line.values[i + 1],
                    OperatorPart2::Mul  => result *= line.values[i + 1],
                    OperatorPart2::Concat => {
                        let mut tmp = result.to_string();
                        tmp += &line.values[i + 1].to_string();
                        result = tmp.parse().unwrap()
                    }
                }
            }
            if result == line.target {
                sum += line.target;
                break;
            }
        }
    }
    sum
}