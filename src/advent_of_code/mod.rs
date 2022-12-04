pub mod year_2022 {

    pub mod day_01 {
        use std::io::BufRead;

        pub fn part1() {
            let (max, _) = std::io::stdin()
                .lock()
                .lines()
                .fold((0, 0), |(max, buffer), line| {
                    let line = line.unwrap();

                    if line.is_empty() && buffer > max {
                        (buffer, 0)
                    } else if line.is_empty() {
                        (max, 0)
                    } else {
                        let cals: u128 = line.parse().unwrap();
                        (max, buffer + cals)
                    }
                });
            println!("{}", max);
        }

        pub fn part2(input: &[u128]) -> u128 {
            input[input.len() - 3..].iter().sum()
        }

        pub fn input() -> Vec<u128> {
            let (mut max, buffer) = std::io::stdin()
            .lock()
            .lines()
            .fold((vec![], 0), |(mut max, buffer), line| {
                let line = line.unwrap();

                if line.is_empty() {
                    max.push(buffer);
                    (max, 0)
                } else {
                    let c: u128 = line.parse().unwrap();
                    (max, buffer + c)
                }
            });

            max.push(buffer);
            max.sort();
            max
        }
    
    
    }

    pub mod day_02 {
        use std::io::BufRead;
        use anyhow::{anyhow, bail, Result};

        #[derive(Debug, Clone, Copy)]
        enum Shape {
            Rock,
            Paper,
            Scissors,
        }

        impl Shape {
            fn points(self) -> u128 {
                match self {
                    Shape::Rock => 1,
                    Shape::Paper => 2,
                    Shape::Scissors => 3,
                }
            }
        }

        pub fn input() -> Result<Vec<Round>> {
            std::io::stdin()
                .lock()
                .lines()
                .map(|line| -> Result<_>{
                    use Shape::*;

                    let line = line?;
                    let mut split = line.split(' ');

                    let player1 = match split.next().ok_or_else(|| anyhow!("No play enemy"))? {
                        "A" => Rock,
                        "B" => Paper,
                        "C" => Scissors,
                        _ => bail!("Invalid play")
                    };

                    let player2 = match split.next().ok_or_else(|| anyhow!("No play enemy"))? {
                        "X" => Rock,
                        "Y" => Paper,
                        "Z" => Scissors,
                        _ => bail!("Invalid play")
                    };

                    if split.next().is_some() {
                        bail!("Invalid")
                    }

                    Ok(Round {
                        enemy: player1,
                        me: player2,
                    })

                }).collect()
        }

        enum Outcome {
            Win,
            Lose,
            Draw,
        }

        impl Outcome {
            fn points(self) -> u128 {
                match self {
                    Outcome::Win => 6,
                    Outcome::Lose => 0,
                    Outcome::Draw => 3,
                }
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Round {
            me: Shape,
            enemy: Shape,
        }

        impl Round {
            fn outcome(self) -> Outcome {
                use Shape::*;
                use Outcome::*;

                match (self.me, self.enemy) {
                    (Rock, Rock) => Draw,
                    (Rock, Paper) => Lose,
                    (Rock, Scissors) => Win,
                    (Paper, Rock) => Win,
                    (Paper, Paper) => Draw,
                    (Paper, Scissors) => Lose,
                    (Scissors, Rock) => Lose,
                    (Scissors, Paper) => Win,
                    (Scissors, Scissors) => Draw,
                }
            }

            fn points(self) -> u128 {
                self.outcome().points() + self.me.points()
            }
        }

        pub fn part1(input: &[Round]) -> u128 {
            input.iter().map(|round| round.points()).sum()
        }


    }

}
