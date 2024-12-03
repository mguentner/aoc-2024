use core::str;

enum ParseState1 {
    Start,
    MulM,
    MulU,
    MulL,
    Arg1,
    Arg2,
}

fn execute_part1(expression: &str) -> i64 {
    let mut parse_state = ParseState1::Start;
    let mut sum = 0i64;
    let mut arg1 = String::new();
    let mut arg2 = String::new();
    for i in expression.chars() {
        match parse_state {
            ParseState1::Start => {
                if i == 'm' {
                    arg1.clear();
                    arg2.clear();
                    parse_state = ParseState1::MulM;
                }
            }
            ParseState1::MulM => {
                if i == 'u' {
                    parse_state = ParseState1::MulU;
                } else {
                    parse_state = ParseState1::Start;
                }
            }
            ParseState1::MulU => {
                if i == 'l' {
                    parse_state = ParseState1::MulL;
                } else {
                    parse_state = ParseState1::Start;
                }
            }
            ParseState1::MulL => {
                if i == '(' {
                    parse_state = ParseState1::Arg1;
                } else {
                    parse_state = ParseState1::Start;
                }
            }
            ParseState1::Arg1 => {
                if "0123456789".contains(i) {
                    arg1.push(i);
                } else if i == ',' {
                    parse_state = ParseState1::Arg2;
                } else {
                    parse_state = ParseState1::Start;
                }
            }
            ParseState1::Arg2 => {
                if "0123456789".contains(i) {
                    arg2.push(i);
                } else if i == ')' {
                    let arg1_int  = arg1.parse::<i64>().unwrap();
                    let arg2_int  = arg2.parse::<i64>().unwrap();
                    sum += arg1_int * arg2_int;
                    parse_state = ParseState1::Start;
                } else {
                    parse_state = ParseState1::Start;
                }
            }
        }
    }
    sum
}

#[derive(Debug)]
enum ParseState2 {
    Start,
    MulM,
    MulU,
    MulL,
    Arg1,
    Arg2,
    DoD,
    DoO,
    DoN,
    DoAp,
    DoT,
    DoBrackDo,
    DoBrackDont,
}

fn execute_part2(expression: &str) -> i64 {
    let mut parse_state = ParseState2::Start;
    let mut sum = 0i64;
    let mut arg1 = String::new();
    let mut arg2 = String::new();
    let mut sum_next = true;
    for c in expression.chars() {
        match parse_state {
            ParseState2::Start => {
                if c == 'm' {
                    arg1.clear();
                    arg2.clear();
                    parse_state = ParseState2::MulM;
                }
                if c == 'd' {
                    parse_state = ParseState2::DoD;
                }
            }
            ParseState2::MulM => {
                if c == 'u' {
                    parse_state = ParseState2::MulU;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::MulU => {
                if c == 'l' {
                    parse_state = ParseState2::MulL;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::MulL => {
                if c == '(' {
                    parse_state = ParseState2::Arg1;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::Arg1 => {
                if "0123456789".contains(c) {
                    arg1.push(c);
                } else if c == ',' {
                    parse_state = ParseState2::Arg2;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::Arg2 => {
                if "0123456789".contains(c) {
                    arg2.push(c);
                } else if c == ')' {
                    let arg1_int  = arg1.parse::<i64>().unwrap();
                    let arg2_int  = arg2.parse::<i64>().unwrap();
                    if sum_next {
                        sum += arg1_int * arg2_int;
                    }
                    parse_state = ParseState2::Start;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::DoD => {
                if c == 'o' {
                    parse_state = ParseState2::DoO;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::DoO => {
                if c == '(' {
                    parse_state = ParseState2::DoBrackDo;
                } else if c == 'n'  {
                    parse_state = ParseState2::DoN;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::DoBrackDo => {
                if c == ')'  {
                    sum_next = true;
                }
                parse_state = ParseState2::Start;
            }
            ParseState2::DoN => {
                if c == '\'' {
                    parse_state = ParseState2::DoAp;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::DoAp => {
                if c == 't' {
                    parse_state = ParseState2::DoT;
                } else {
                    parse_state = ParseState2::Start;
 
                }
            }
            ParseState2::DoT => {
                if c == '(' {
                    parse_state = ParseState2::DoBrackDont;
                } else {
                    parse_state = ParseState2::Start;
                }
            }
            ParseState2::DoBrackDont => {
                if c == ')' {
                    sum_next = false;
                }
                parse_state = ParseState2::Start;
            }
        }
    }
    sum
}

fn main() {
    let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let example2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let input_data = include_bytes!("../input.txt");

    println!("example = {}\n", execute_part1(example));
    println!("input = {}\n", execute_part1(str::from_utf8(input_data).unwrap()));
    println!("example = {}\n", execute_part2(example2));
    println!("input = {}\n", execute_part2(str::from_utf8(input_data).unwrap()));
}
