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


fn main() {
    println!("The score in problem 1 is {}", day3::solve_problem_1("data/day3.txt"));
    println!("The score in problem 2 is {}", day3::solve_problem_2("data/day3.txt"));
}
