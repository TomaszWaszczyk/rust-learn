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
}
