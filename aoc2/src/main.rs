use util::{input_file_path, read_lines};

// Part 1
// mod rps {
//     #[derive(Debug)]
//     pub enum RPS {
//         Rock,
//         Paper,
//         Scissors,
//     }
    
//     impl From<&str> for RPS {
//         fn from(c: &str) -> Self {
//             match c {
//                 "A" | "X" => RPS::Rock,
//                 "B" | "Y" => RPS::Paper,
//                 "C" | "Z" => RPS::Scissors,
//                 _ => panic!("str {} does not match a valid rock-paper-scissors option", c),
//             }
//         }
//     }

//     impl From<&RPS> for i32 {
//         fn from(opt: &RPS) -> i32 {
//             match opt {
//                 RPS::Rock => 0,
//                 RPS::Paper => 1,
//                 RPS::Scissors => 2,
//             }
//         }
//     }

//     impl RPS {
//         fn score_by_type(&self) -> usize {
//             match self {
//                 RPS::Rock => 1,
//                 RPS::Paper => 2,
//                 RPS::Scissors => 3,
//             }
//         }

//         pub fn score(&self, other: &RPS) -> usize {
//             let diff = (<&RPS as Into<i32>>::into(self) - <&RPS as Into<i32>>::into(other)).rem_euclid(3);
//             let score_against = match diff {
//                 0 => 3,
//                 1 => 6,
//                 2 => 0,
//                 _ => panic!("{diff}"),
//             };

//             score_against + self.score_by_type()
//         }
//     }
// }

mod rps {
    #[derive(Debug)]
    pub enum RPS {
        Rock,
        Paper,
        Scissors,
    }
    
    impl From<&str> for RPS {
        fn from(c: &str) -> Self {
            match c {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!("str {} does not match a valid rock-paper-scissors option", c),
            }
        }
    }

    impl From<&RPS> for i32 {
        fn from(opt: &RPS) -> i32 {
            match opt {
                RPS::Rock => 0,
                RPS::Paper => 1,
                RPS::Scissors => 2,
            }
        }
    }

    impl From<i32> for RPS {
        fn from(val: i32) -> RPS {
            match val {
                0 => RPS::Rock,
                1 => RPS::Paper,
                2 => RPS::Scissors,
                _ => panic!("i32 {} does not match a valid rock-paper-scissors option", val),
            }
        }
    }

    impl RPS {
        pub fn determine_response(&self, result_str: &str) -> Self {
            let diff = match result_str {
                "X" => -1,
                "Y" => 0,
                "Z" => 1,
                _ => panic!("str {} does not match a valid win/loss/draw option", result_str),
            };
            let response = (<&RPS as Into<i32>>::into(self) + diff).rem_euclid(3);
            response.into()
        }

        fn score_by_type(&self) -> usize {
            match self {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scissors => 3,
            }
        }

        pub fn score(&self, other: &RPS) -> usize {
            let diff = (<&RPS as Into<i32>>::into(self) - <&RPS as Into<i32>>::into(other)).rem_euclid(3);
            let score_against = match diff {
                0 => 3,
                1 => 6,
                2 => 0,
                _ => panic!("{diff}"),
            };

            score_against + self.score_by_type()
        }
    }
}

use rps::RPS;

fn main() {
    let lines = read_lines(input_file_path()).expect("Could not parse input file lines");

    let mut score = 0;
    for line in lines {
        let line = line.expect("Could not unwrap line");
        let c: Vec<&str> = line.split_whitespace().collect();
        if c.len() != 2 {
            panic!("Didn't find just 2 chars in the line");
        }

        // Part 1
        // let hands: Vec<RPS> = c.into_iter().map(|h| RPS::from(h)).collect();
        // score += hands[1].score(&hands[0]);

        // Part 2
        let elf_hand = RPS::from(c[0]);
        let my_hand = elf_hand.determine_response(c[1]);
        score += my_hand.score(&elf_hand);
    }

    print!("Total score: {}", score);
}
