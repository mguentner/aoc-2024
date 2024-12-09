#[derive(Clone, Copy, Debug)]
enum DiskSpace {
    File { size: u8, id: u64 },
    Free(u8),
}

fn create_layout(input: &str) -> Vec<DiskSpace> {
    let mut result = Vec::new();
    for (idx, value) in input.chars().enumerate() {
        let value_u8: u8 = match value.to_digit(10) {
            Some(value_u8) => value_u8.try_into().unwrap(),
            None => {
                continue;
            }
        };
        if value_u8 == 0 {
            continue;
        }
        if idx % 2 == 0 {
            // file
            result.push(DiskSpace::File {
                size: value_u8,
                id: idx as u64 / 2,
            });
        } else {
            result.push(DiskSpace::Free(value_u8));
        }
    }
    result
}

fn defragment_part1(layout: &Vec<DiskSpace>) -> Vec<DiskSpace> {
    let mut result = layout.clone();
    let (free_idx, free_space) = result
        .iter()
        .enumerate()
        .find(|(_, space)| matches!(space, DiskSpace::Free(_)))
        .map(|(x, y)| (x, *y))
        .unwrap();
    let (file_idx, file_space) = result
        .iter()
        .enumerate()
        .rev()
        .find(|(_, space)| matches!(space, DiskSpace::File { .. }))
        .map(|(x, y)| (x, *y))
        .unwrap();

    match (&free_space, &file_space) {
        (DiskSpace::Free(free_size), DiskSpace::File { size, id }) if size > free_size => {
            result[free_idx] = DiskSpace::File {
                size: *free_size,
                id: *id,
            };
            result[file_idx] = DiskSpace::File {
                size: size - free_size,
                id: *id,
            };
            result.insert(file_idx + 1, DiskSpace::Free(*free_size));
        }
        (DiskSpace::Free(free_size), DiskSpace::File { size, id }) if size == free_size => {
            result[free_idx] = DiskSpace::File {
                size: *size,
                id: *id,
            };
            result[file_idx] = DiskSpace::Free(*size);
        }
        (DiskSpace::Free(free_size), DiskSpace::File { size, id }) if size < free_size => {
            result[free_idx] = DiskSpace::File {
                size: *size,
                id: *id,
            };
            result[file_idx] = DiskSpace::Free(*size);
            result.insert(free_idx + 1, DiskSpace::Free(*free_size - *size));
        }
        _ => println!("unexpected combo"),
    }
    result
}

fn move_id(layout: &Vec<DiskSpace>, move_id: u64) -> Vec<DiskSpace> {
    let mut result = layout.clone();
    let (file_idx, file) = layout
        .iter()
        .enumerate()
        .find(|(_, space)| match space {
            DiskSpace::File { size, id } => *id == move_id,
            _ => false,
        })
        .unwrap();
    if file_idx == 0 {
        return result;
    }
    for (free_idx, space) in layout.iter().enumerate() {
        if file_idx < free_idx {
            break;
        }
        match (space, file) {
            (DiskSpace::Free(free_size), DiskSpace::File { size, id }) if *size < *free_size => {
                result[free_idx] = DiskSpace::File {
                    size: *size,
                    id: *id,
                };
                result[file_idx] = DiskSpace::Free(*size);
                result.insert(free_idx + 1, DiskSpace::Free(*free_size - *size));
                break;
            }
            (DiskSpace::Free(free_size), DiskSpace::File { size, id }) if *size == *free_size => {
                result[free_idx] = DiskSpace::File {
                    size: *size,
                    id: *id,
                };
                result[file_idx] = DiskSpace::Free(*size);
                break;
            }
            _ => continue,
        }
    }
    result
}

fn defragment_part2(layout: &Vec<DiskSpace>) -> Vec<DiskSpace> {
    let mut to_move = layout
        .iter()
        .filter_map(|x| match x {
            DiskSpace::File { size, id } => Some(*id),
            _ => None,
        })
        .collect::<Vec<u64>>();
    to_move.sort();
    to_move.reverse();

    let mut result = layout.clone();
    for to_move_id in to_move {
        result = move_id(&result, to_move_id);
    }

    result
}

fn is_defragmented(layout: &Vec<DiskSpace>) -> bool {
    let free_idx = layout
        .iter()
        .enumerate()
        .find(|(_, space)| matches!(space, DiskSpace::Free(_)))
        .map(|(x, _)| x)
        .unwrap();
    let file_idx = layout
        .iter()
        .enumerate()
        .rev()
        .find(|(_, space)| matches!(space, DiskSpace::File { .. }))
        .map(|(x, _)| x)
        .unwrap();
    if free_idx < file_idx {
        return false;
    }
    true
}

fn debug_layout(layout: &Vec<DiskSpace>) {
    for space in layout {
        match space {
            DiskSpace::Free(size) => {
                for i in 0..*size {
                    print!(".")
                }
            }
            DiskSpace::File { id, size } => {
                for i in 0..*size {
                    print!("{id}")
                }
            }
        };
    }
    print!("\n")
}

fn checksum_layout(layout: &Vec<DiskSpace>) -> u64 {
    let mut sum = 0;
    let mut pos: u64 = 0;
    for (idx, space) in layout.iter().enumerate() {
        match space {
            DiskSpace::File { size, id } => {
                for i in 0..*size {
                    sum += id * (pos + u64::from(i));
                }
                pos += u64::from(*size);
            }
            DiskSpace::Free(size) => {
                pos += u64::from(*size);
            }
        };
    }
    sum
}

fn main() {
    let example = "2333133121414131402";
    let example_layout = create_layout(&example);

    let input = include_str!("../input.txt");
    let input_layout = create_layout(&input);

    let mut tmp_example = example_layout.clone();
    while !is_defragmented(&tmp_example) {
        tmp_example = defragment_part1(&tmp_example);
    }

    let mut tmp_input = input_layout.clone();
    while !is_defragmented(&tmp_input) {
        tmp_input = defragment_part1(&tmp_input);
    }

    let defragmented_example2 = defragment_part2(&example_layout);
    let defragmented_input2 = defragment_part2(&input_layout);

    println!("checksum example: {}", checksum_layout(&tmp_example));
    println!("checksum input: {}", checksum_layout(&tmp_input));
    
    println!("checksum example: {}", checksum_layout(&defragmented_example2));
    println!("checksum input: {}", checksum_layout(&defragmented_input2));
}
