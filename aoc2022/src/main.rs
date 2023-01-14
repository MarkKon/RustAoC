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

pub mod day2 {
    enum Move{
        Rock,
        Paper,
        Scissors,
    }

    enum Outcome{
        Win,
        Lose,
        Draw,
    }

    fn parse_line(line: &str) -> (Move, Move) {
        let line = line.trim();
        let mut moves = line.split(" ");
        let player1 = match moves.next() {
            Some("A") => Move::Rock,
            Some("B") => Move::Paper,
            Some("C") => Move::Scissors,
            _ => panic!("Invalid move"),
        };
        let player2 = match moves.next() {
            Some("X") => Move::Rock,
            Some("Y") => Move::Paper,
            Some("Z") => Move::Scissors,
            _ => panic!("Invalid move"),
        };
        return (player1, player2)
    }

    fn move_points(a_move: &Move) -> u32 {
        match a_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn move_outcome(player1: Move, player2: Move) -> Outcome {
        match (player1, player2) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Win,
            (Move::Rock, Move::Scissors) => Outcome::Lose,
            (Move::Paper, Move::Rock) => Outcome::Lose,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Lose,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }

    fn outcome_points(outcome: Outcome) -> u32 {
        match outcome {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn total_points(player1: Move, player2: Move) -> u32 {
        let mp = move_points(&player2);
        let mo = move_outcome(player1, player2);
        let op = outcome_points(mo);
        mp + op
    }

    fn line_to_points(line: &str) -> u32 {
        let (player1, player2) = parse_line(line);
        total_points(player1, player2)
    }

    fn parse_outcome_line(line: &str) -> (Move, Outcome) {
        let line = line.trim();
        let mut moves = line.split(" ");
        let player1 = match moves.next() {
            Some("A") => Move::Rock,
            Some("B") => Move::Paper,
            Some("C") => Move::Scissors,
            _ => panic!("Invalid move"),
        };
        let outcome = match moves.next() {
            Some("X") => Outcome::Lose,
            Some("Y") => Outcome::Draw,
            Some("Z") => Outcome::Win,
            _ => panic!("Invalid outcome"),
        };
        return (player1, outcome)
    }

    fn match_move(enemy: &Move, expected_outcome: Outcome) -> Move {
        match (enemy, expected_outcome) {
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
        }
    }

    fn line_to_outcome_points(line: &str) -> u32 {
        let (enemy, expected_outcome) = parse_outcome_line(line);
        let my_move = match_move(&enemy, expected_outcome);
        total_points(enemy, my_move)
    }

    pub fn solve_problem_1(path: &str) -> u32 {
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        let lines = input.split("\r\n");
        lines.map(line_to_points).sum()
    }


    pub fn solve_problem_2(path: &str) -> u32 {
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        let lines = input.split("\r\n");
        lines.map(line_to_outcome_points).sum()
    }
}

pub mod day3 {
    fn priority(c: char) ->u8{
        match c {
            'a'..='z' => c as u8 - 'a' as u8 + 1,
            'A'..='Z' => c as u8 - 'A' as u8 + 27,
            _ => panic!("Invalid character"),
        }
    }

    fn parse_line(line: &str) -> (Vec<char>, Vec<char>) {
        let line = line.trim();
        // split at the midpoint
        let midpoint = match line.len(){
            x if x % 2 == 0 => x / 2,
            _ => panic!("Invalid line length"),
        };
        let left_vec: Vec<char> = line.chars().take(midpoint).collect();
        let right_vec: Vec<char> = line.chars().skip(midpoint).collect();
        (left_vec, right_vec)
    }

    fn intersect(vec1: &Vec<char>, vec2: &Vec<char>) -> char {
        // calculate the (unique) character that is in both vectors
        vec1.iter().find(|&&x| vec2.contains(&x)).unwrap().clone()
    }

    fn intersect3(vec1: &Vec<char>, vec2: &Vec<char>, vec3: &Vec<char>) -> char {
        // calculate the (unique) character that is in all three vectors
        vec1.iter().find(|&&x| vec2.contains(&x) && vec3.contains(&x)).unwrap().clone()
    }

    pub fn solve_problem_1(path: &str) -> u32 {
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        let lines = input.split("\r\n");
        lines.map(|line| {
            let (left, right) = parse_line(line);
            let c = intersect(&left, &right);
            let p = priority(c);
            p as u32
        }).sum()
    }

    pub fn solve_problem_2(path: &str) -> u32{
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        let lines = input.split("\r\n");
        // split into triples
        let binding = lines.collect::<Vec<&str>>();
        let triples = binding.chunks(3);
        triples.map(|triple| {
            let triple= triple.iter()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            let c = intersect3(&triple[0], &triple[1], &triple[2]);
            priority(c) as u32
        }).sum()
    }
}

pub mod day4 {
    struct Range{
        lower: u32,
        upper: u32,
    }

    fn safe_parse(s: &str) -> Option<u32>{
        match s.parse::<u32>(){
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    fn parse_range(s: &str) -> Range{
        let s = s.trim();
        let mut split = s.split("-");
        let lower = split.next().unwrap();
        let upper = split.next().unwrap();
        let lower = safe_parse(lower).unwrap();
        let upper = safe_parse(upper).unwrap();
        Range{lower, upper}
    }

    fn parse_line(line: &str) -> (Range, Range){
        let line = line.trim();
        let both = line.split(",").collect::<Vec<&str>>();
        if both.len() != 2{
            panic!("Invalid line, should be two ranges separated by a comma");
        };
        let (left, right) = (both[0], both[1]);
        let left = parse_range(left);
        let right = parse_range(right);
        (left, right)
    }

    fn contained(r1: &Range, r2: &Range) -> bool{
        r1.lower >= r2.lower && r1.upper <= r2.upper || r2.lower >= r1.lower && r2.upper <= r1.upper
    }

    fn overlap_some(r1: &Range, r2: &Range) -> bool{
        r1.upper >= r2.lower && r1.upper <= r2.upper || r2.upper >= r1.lower && r2.upper <= r1.upper
    }

    pub fn solve_problem_1(path: &str) -> u32{
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        let lines = input.split("\r\n");
        lines.map(parse_line).filter(|(r1, r2)| contained(r1, r2)).count() as u32
    }

    pub fn solve_problem_2(path: &str) -> u32{
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        let lines = input.split("\r\n");
        lines.map(parse_line).filter(|(r1, r2)| overlap_some(r1, r2)).count() as u32
    }
}

pub mod day5 {
    // wrap Vec<Vec<char>> to make it safe to use
    struct Stock {
        stock_field: Vec<Vec<char>>,
    }

    // implement methods for Stock
    impl Stock {
        fn stock_move(from: u8, to: u8, stock: &mut Stock){
            let c = stock.stock_field[from as usize].pop().unwrap();
            stock.stock_field[to as usize].push(c);
        }

        fn stock_move_multiple(from: u8, to: u8, num: u8, stock: &mut Stock){
            // move num crates from from to to and preserve order
            let mut move_crates = Vec::new();
            for _ in 0..num{
                move_crates.push(stock.stock_field[from as usize].pop().unwrap());
            }
            move_crates.reverse();
            for c in move_crates{
                stock.stock_field[to as usize].push(c);
            }
        }

        fn top(stock: &Stock) -> Vec<char>{
            stock.stock_field.iter().map(|s| s.last().unwrap().clone()).collect::<Vec<char>>()
        }

        fn from_path(path: &str) -> Stock{
            let input = match std::fs::read_to_string(path) {
                Ok(input) => input,
                Err(error) => panic!("Error reading input file: {}", error),
            };
            // take out everything after the double newline
            let input = input.split("\r\n\r\n").next().unwrap();
            let mut lines = input.split("\r\n").collect::<Vec<&str>>();
    
            let first_line = lines.pop().unwrap().trim();
            let num_crates = first_line.split(" ").last().unwrap();
            let num_crates = num_crates.parse::<u8>().unwrap();
    
            // reverse the order in lines
            lines.reverse();
            let clone = lines.clone();
            let num_lines = clone.len();
            // get the last number in line
            
            // reserve space for num_crates crates of size num_lines
            let mut crates: Vec<Vec<char>> = Vec::with_capacity(num_crates as usize);
            for _ in 0..num_crates{
                crates.push(Vec::with_capacity(num_lines));
            };
            for line in lines.iter(){
                // split at every 4th character
                let chars = line.chars().collect::<Vec<char>>();
                let copy = chars.clone();
                let chunked = copy.chunks(4);
                for (i, chunk) in chunked.enumerate(){
                    let c = chunk[1];
                    if c != ' '{
                        crates[i].push(c);
                    };
                };
            };
            Stock{stock_field: crates}
        }
    }

    fn parse_move(s: &str) -> (u8,u8, u8){
        // line like "Move x from y to z" should be parsed to (x, y, z)
        let s = s.trim();
        let mut split = s.split(" ");
        let _ = split.next().unwrap();
        let x = split.next().unwrap();
        let _ = split.next().unwrap();
        let y = split.next().unwrap();
        let _ = split.next().unwrap();
        let z = split.next().unwrap();
        let x = x.parse::<u8>().unwrap();
        let y = y.parse::<u8>().unwrap();
        let z = z.parse::<u8>().unwrap();
        (x, y, z)
    }

    pub fn solve_problem_1(path: &str) -> Vec<char>{
        let mut stock = Stock::from_path(path);
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        let lines = input.split("\r\n\r\n").collect::<Vec<&str>>();
        let moves = lines[1].split("\r\n");
        for line in moves{
            let (x, y, z) = parse_move(line);
            for _ in 0..x{
                Stock::stock_move(y-1, z-1, &mut stock);
            };
        };
        Stock::top(&stock)
    }

    pub fn solve_problem_2(path: &str) -> Vec<char>{
        let mut stock = Stock::from_path(path);
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        let lines = input.split("\r\n\r\n").collect::<Vec<&str>>();
        let moves = lines[1].split("\r\n");
        for line in moves{
            let (x, y, z) = parse_move(line);
            Stock::stock_move_multiple(y-1, z-1, x, &mut stock);
        };
        Stock::top(&stock)
    }

}

pub mod day6 {
    fn check_packet(packet: &Vec<char>)-> bool{
        // check if all characters in packet are different
        let mut chars = packet.clone();
        chars.sort();
        chars.dedup();
        chars.len() == packet.len()
    }

    pub fn solve_problem_1(path: &str) -> Option<usize> {
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        for i in 0..input.len()-4{
            let s = &input[i..i+4];
            if check_packet(&s.chars().collect::<Vec<char>>()){
                return Some(i+4);
            };
        };
        None
    }

    pub fn solve_problem_2(path: &str) -> Option<usize> {
        let input = match std::fs::read_to_string(path) {
            Ok(input) => input,
            Err(error) => panic!("Error reading input file: {}", error),
        };
        for i in 0..input.len()-14{
            let s = &input[i..i+14];
            if check_packet(&s.chars().collect::<Vec<char>>()){
                return Some(i+14);
            };
        };
        None
    }
}


// use std::env;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    // debug read_crates
    println!("The score in problem 1 is {:?}", day6::solve_problem_1("data/day6.txt"));
    println!("The score in problem 2 is {:?}", day6::solve_problem_2("data/day6.txt"));
}
