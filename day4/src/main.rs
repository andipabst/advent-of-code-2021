fn main() {
    let mut lines = include_str!("../input.txt").split("\n\n");

    let bingo_numbers: Vec<u32> = lines.next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    let mut bingo_boards: Vec<Board> = lines.map(|b| {
        let mut numbers = [[0; 5]; 5];
        for (rownum, line) in b.lines().enumerate() {
            for (colnum, number) in line.split_whitespace().enumerate() {
                numbers[rownum][colnum] = number.parse().unwrap()
            }
        }
        return create_board(numbers);
    }).collect();

    //println!("{:?}", bingo_numbers);
    //println!("{:?}", bingo_boards);

    let mut last_answer: Option<u32> = None;

    for number in bingo_numbers.iter() {
        for board in bingo_boards.iter_mut() {
            if !board.has_won() {
                board.mark_number(*number);

                if board.has_won() {
                    if last_answer.is_none() {
                        println!("Answer for Part 1: {}", board.sum_of_unmarked() * number);
                    }
                    last_answer = Some(board.sum_of_unmarked() * number);
                    //println!("Got a Winner!\nNumber: {}\nBoard: {:?}", number, board);
                }
            }
        }
    }

    println!("Answer for Part 2: {}", last_answer.unwrap_or(0));
}

#[derive(Debug)]
struct Board {
    numbers: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl Board {
    fn mark_number(&mut self, number: u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == number {
                    self.marks[i][j] = true
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..5 {
            // check rows            
            if self.marks[i][0] && self.marks[i][1] && self.marks[i][2] && self.marks[i][3] && self.marks[i][4] {
                return true;
            }
            // check columns
            if self.marks[0][i] && self.marks[1][i] && self.marks[2][i] && self.marks[3][i] && self.marks[4][i] {
                return true;
            }
        }

        return false;
    }

    fn sum_of_unmarked(&self) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marks[i][j] {
                    sum += self.numbers[i][j]
                }
            }
        }

        return sum;
    }
}

impl Board {}

fn create_board(numbers: [[u32; 5]; 5]) -> Board {
    return Board {
        numbers,
        marks: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ],
    };
} 