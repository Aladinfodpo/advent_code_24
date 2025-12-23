use std::env;
use std::fs;

struct DoubleList{
    left : Vec<u32>,
    right : Vec<u32>
}

impl DoubleList {
    fn from_string(string : String) -> DoubleList {
        let mut left = Vec::new();
        let mut right = Vec::new();

        string.replace("   ", " ").lines().for_each(|line|{
            let mut it = line.split(" ");
            left.push(it.next().unwrap().parse().unwrap());
            right.push(it.next().unwrap().parse().unwrap());
        });

        DoubleList { left, right }
    }

    fn sort(&mut self){
        self.left.sort();
        self.right.sort();
    }

    fn get_sum_distance(&self) -> u32 {
        (0..self.left.len()).fold(0, |res, index|{res + self.left[index].abs_diff(self.right[index])})
    }

    fn get_p2(&self) -> u32{
        (0..self.left.len()).fold(0, |res, index|{res + self.right.iter().filter(|righty| {**righty == self.left[index]}).count() as u32 * self.left[index]})
    }
}

fn get_p1(string: String) -> u32 {
    let mut lists = DoubleList::from_string(string);
    lists.sort();
    lists.get_sum_distance()
}

fn get_p2(string: String) -> u32 {
    DoubleList::from_string(string).get_p2()
}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    println!("P1: {}", get_p1(get_file()));
    println!("P2: {}", get_p2(get_file()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let string = concat!(
            "3   4\n",
            "4   3\n",
            "2   5\n",
            "1   3\n",
            "3   9\n",
            "3   3\n",
        ).to_string();
        assert_eq!(get_p1(string.clone()), 11);
        assert_eq!(get_p2(string.clone()), 31);
    }
}