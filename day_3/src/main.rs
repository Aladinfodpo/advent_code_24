use core::num;
use std::env;
use std::fs;

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    println!("Results: {}", compute_muls(get_file()));
}

enum ParsingState{
    FunctionName(usize),
    FirstNumber(u32),
    SecondNumber(u32, u32)
}

fn compute_muls(string: String)-> u32{
    let mut state = ParsingState::FunctionName(0);
    let mut res = 0;

    for c in string.chars(){
        match &state {
            ParsingState::FunctionName(index) => {
                let parsing = "mul(";
                let parsed_matching = parsing.chars().nth(*index).unwrap();
                match c {
                    'm' => {state = ParsingState::FunctionName(1)},
                    val if val == parsed_matching => {
                        state = if *index == parsing.len() - 1 {
                            ParsingState::FirstNumber(0)
                        }else{
                            ParsingState::FunctionName(index + 1)
                        };
                    },
                    _ => {state = ParsingState::FunctionName(0)},
                }

            },
            ParsingState::FirstNumber(num) => {
                match c {
                    ',' => {state = ParsingState::SecondNumber(*num, 0)},
                    'm' => {state = ParsingState::FunctionName(1)},
                    _ => {
                        if c.is_ascii_digit() {
                            state = ParsingState::FirstNumber(*num * 10 + c.to_string().parse::<u32>().unwrap())
                        } else {
                            state = ParsingState::FunctionName(0)
                        }
                    }
                }
            },
            ParsingState::SecondNumber(first, second) => {
                match c {
                    ')' => {res += first * second; state = ParsingState::FunctionName(0) },
                    'm' => {state = ParsingState::FunctionName(1)},
                    _ => {
                        if c.is_ascii_digit() {
                            state = ParsingState::SecondNumber(*first, *second * 10 + c.to_string().parse::<u32>().unwrap())
                        } else {
                            state = ParsingState::FunctionName(0)
                        }
                    }
                }
            }

        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let string = concat!(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        ).to_string();
        assert_eq!(compute_muls(string), 161);
    }
}