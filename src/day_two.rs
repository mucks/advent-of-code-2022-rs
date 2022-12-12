use crate::util::get_input;

// RockPaperScissor
enum RPS {
    Rock,
    Paper,
    Scissor,
}

impl RPS {
    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
    fn from_me(me: &str) -> Self {
        match me {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissor,
            _ => panic!("invalid input in fn RPS::from_me: {}", me),
        }
    }
    fn from_opponent(op: &str) -> Self {
        match op {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissor,
            _ => panic!("invalid input in fn RPS::from_opponent: {}", op),
        }
    }

    fn play_against_part_two(&self, my_outcome: Outcome) -> u32 {
        use Outcome::*;
        use RPS::*;
        let opponent = self;

        let rps_val = match opponent {
            Rock => match my_outcome {
                Lose => Scissor.value(),
                Draw => Rock.value(),
                Win => Paper.value(),
            },
            Paper => match my_outcome {
                Lose => Rock.value(),
                Draw => Paper.value(),
                Win => Scissor.value(),
            },
            Scissor => match my_outcome {
                Draw => Scissor.value(),
                Lose => Paper.value(),
                Win => Rock.value(),
            },
        };
        rps_val + my_outcome.value()
    }

    fn play_against(&self, op: &Self) -> u32 {
        use RPS::*;

        let v = self.value();
        let lose = 0;
        let draw = 3;
        let win = 6;

        match self {
            Rock => match op {
                Rock => draw + v,
                Paper => lose + v,
                Scissor => win + v,
            },
            Paper => match op {
                Rock => win + v,
                Paper => draw + v,
                Scissor => lose + v,
            },
            Scissor => match op {
                Rock => lose + v,
                Paper => win + v,
                Scissor => draw + v,
            },
        }
    }
}

enum Outcome {
    Lose,
    Win,
    Draw,
}

impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
    fn from_str(me: &str) -> Self {
        match me {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid input in fn Outcome::from_str: {}", me),
        }
    }
}

fn part_one(input: &str) {
    let mut results = vec![];
    for line in input.split('\n') {
        let split: Vec<&str> = line.split_whitespace().collect();

        let opponent = RPS::from_opponent(split[0]);
        let me = RPS::from_me(split[1]);
        let result = me.play_against(&opponent);
        results.push(result);
    }
    println!("Part one: Total Score: {}", results.iter().sum::<u32>());
}

fn part_two(input: &str) {
    let mut results = vec![];
    for line in input.split('\n') {
        let split: Vec<&str> = line.split_whitespace().collect();

        let opponent = RPS::from_opponent(split[0]);
        let my_outcome = Outcome::from_str(split[1]);
        let result = opponent.play_against_part_two(my_outcome);
        results.push(result);
    }
    println!("Part two: Total Score: {}", results.iter().sum::<u32>());
}

pub fn run() {
    let input = get_input("day_two");
    part_one(&input);
    part_two(&input);
}
