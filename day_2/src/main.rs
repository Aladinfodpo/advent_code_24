use std::env;
use std::fs;

struct Rapport {
    list : Vec<u32>
}

impl Rapport {
    fn is_sorted(& self)-> bool{
        let sign = self.list.last().unwrap() > self.list.first().unwrap();
        self.list.windows(2).fold(true,
             |res, slice|{
                res && (slice[1] > slice[0]) == sign && (slice[0].abs_diff(slice[1]) as i64 - 2).abs() < 2
             })
    }

    fn is_one_left_sorted(& self) -> bool {
        let mut res = false;
        for off_one in 0..self.list.len(){
            res = res || Rapport{ list : (0..self.list.len()).filter(|index|{*index != off_one}).map(|index| {self.list[index]}).collect()}.is_sorted();
        }
        res
    }
}

fn parse_rapport(string : String) -> Vec<Rapport> {
    string.lines().map(
        |line|{
            Rapport{list : line.split(" ").map(
                |number|{
                    number.parse().unwrap()
                }
            ).collect()}
        }
    ).collect()
}

fn get_p1(string : String) -> u32 {
    parse_rapport(string).iter().fold(0, |res, rapport|{res + if rapport.is_sorted(){1}else{0}})
}

fn get_p2(string : String) -> u32 {
    parse_rapport(string).iter().fold(0, |res, rapport|{res + if rapport.is_one_left_sorted(){1}else{0}})
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
            "7 6 4 2 1\n",
            "1 2 7 8 9\n",
            "9 7 6 2 1\n",
            "1 3 2 4 5\n",
            "8 6 4 4 1\n",
            "1 3 6 7 9\n",
        ).to_string();
        assert_eq!(get_p1(string.clone()), 2);
        assert_eq!(get_p2(string.clone()), 4);
    }
}