pub mod day1 {
    pub fn solve_problem_1(path: &str) -> u32 {
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        get_max_sum(&input)
    }

    pub fn solve_problem_2(path: &str) -> u32 {
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        get_top_three(&input)
    }

    fn get_max_sum(input: &str) -> u32 {
        let cells = input.split("\r\n\r\n");
        let max = cells.map(sum_on_cell).max();
        match max {
            Some(max) => max,
            None => 0,
        }
    }

    fn get_top_three(input: &str) -> u32 {
        let cells = input.split("\r\n\r\n");
        let mut nums: Vec<u32> = cells.map(sum_on_cell).collect();
        nums.sort();
        nums.reverse();
        nums.truncate(3);
        nums.iter().sum()
    }

    fn sum_on_cell(cell: &str) -> u32 {
        // println!("cell: {}", cell);
        cell.split("\r\n")
            .map(parse_line)
            .sum()
    }

    fn parse_line(line: &str) -> u32 {
        let line = line.trim();
        match line.parse::<u32>() {
            Ok(num) => num,
            Err(_) => 0,
        }
    }
}




fn main() {
    println!("The max sum is {}", day1::solve_problem_1("data/day1.txt"));
    println!("The sum of top three is {}", day1::solve_problem_2("data/day1.txt"));
}
