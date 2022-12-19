use std::default::Default;

#[derive(Debug, PartialEq, Eq)]
pub struct CrateMove {
    num_crates: usize,
    from: usize,
    to: usize,
}

type CrateStack = Vec<char>;

#[derive(Debug)]
pub struct CargoShip {
    stacks: Vec<CrateStack>,
}

impl Default for CargoShip {
    fn default() -> Self {
        Self {
            stacks: vec![vec![]; 9],
        }
    }
}

impl CargoShip {
    fn move_crates_9000(&mut self, m: &CrateMove) {
        let split_ind = self.stacks[m.from].len() - m.num_crates;
        let mut to_move = self.stacks[m.from].split_off(split_ind);
        to_move.reverse();
        self.stacks[m.to].append(&mut to_move);
    }

    // Repeated code but I'm so sick of this problem at this point I don't care to fix it
    fn move_crates_9001(&mut self, m: &CrateMove) {
        let split_ind = self.stacks[m.from].len() - m.num_crates;
        let mut to_move = self.stacks[m.from].split_off(split_ind);
        self.stacks[m.to].append(&mut to_move);
    }

    fn get_final_orientation(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack[stack.len() - 1])
            .collect::<String>()
    }

    fn push_to_stack(&mut self, stack_ind: usize, c: char) {
        self.stacks[stack_ind].push(c);
    }

    fn flipped(mut self) -> Self {
        for stack in self.stacks.iter_mut() {
            stack.reverse();
        }
        self
    }
}

mod parser {
    use std::num::NonZeroUsize;

    use super::{CargoShip, CrateMove};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{digit1, satisfy},
        combinator::map_res,
        sequence::{delimited, pair, tuple},
        Parser,
    };

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_crate_parse_one() {
            let (tail, c) = crate_parser().parse("[A] ").unwrap();
            assert!(c == Some('A'));
            assert!(tail == "");
        }

        #[test]
        fn test_crate_parse_many() {
            let (_, _) = crate_parser().parse("[A] [B] ").unwrap();
        }

        #[test]
        fn test_crate_parse_empty() {
            let (tail, c) = crate_parser().parse("       \n").unwrap();
            assert!(c.is_none());
            let (tail, c) = crate_parser().parse(tail).unwrap();
            assert!(c.is_none());
            assert!(tail.len() == 0);
        }

        #[test]
        fn test_move_parse() {
            let (tail, m) = move_parser().parse("move 3 from 2 to 1\n").unwrap();
            assert!(tail == "\n");
            assert!(
                m == CrateMove {
                    num_crates: 3,
                    from: 1,
                    to: 0
                }
            );
        }
    }

    fn crate_parser<'a>() -> impl Parser<&'a str, Option<char>, ()> {
        move |input: &'a str| {
            // Get the character in brackets.
            // Delimited by brackets, take 1 char after (or 0 if none available, for newline),
            // and in the middle satisfy a char between A and Z, wrapping in Some.
            let delim_alpha = delimited(
                tag::<&'a str, &'a str, ()>("["),
                map_res(
                    satisfy(|c| u32::from(c) <= u32::from('Z') && u32::from(c) >= u32::from('A')),
                    |c| Ok::<Option<char>, ()>(Some(c)),
                ),
                pair(tag("]"), alt((take(1usize), take(0usize)))),
            );
            // Match delim_alpha or just take 4 chars (allow for 3 b/c newline)
            let mut p = alt((
                delim_alpha,
                map_res(alt((take(4usize), take(3usize))), |_| {
                    Ok::<Option<char>, ()>(None)
                }),
            ));

            // Map parsed
            p.parse(input)
        }
    }

    fn ship_parser<T>() -> impl Parser<T, CargoShip, ()>
    where
        T: Iterator<Item = String>,
    {
        move |mut iter: T| {
            let mut ship = CargoShip::default();

            // Iterate over lines until you hit the empty line
            while let Some(line) = iter.next() {
                let mut line_ref = line.as_str();
                let mut stack = 0;
                let mut crates_parsed = 0;
                while !line_ref.is_empty() {
                    // Parse a crate and add to stack
                    if let Ok((_line_ref, c)) = crate_parser().parse(line_ref) {
                        // Save remainder into line_ref
                        line_ref = _line_ref;
                        // Push char to the proper CrateStack
                        if let Some(c) = c {
                            crates_parsed += 1;
                            ship.push_to_stack(stack, c);
                        }

                        stack += 1;
                    }
                }

                // We will hit this on the line that lists indices
                if crates_parsed == 0 {
                    iter.next().unwrap(); // skip the newline
                    break;
                }
            }

            // Flip crate stacks before returning
            Ok((iter, ship.flipped()))
        }
    }

    fn move_parser<'a>() -> impl Parser<&'a str, CrateMove, ()> {
        use std::str::FromStr;
        move |input| {
            // Parse line as tuple, mapping each digit from a str to a non-zero usize
            let mut tuple_parser = tuple((
                tag("move "),
                map_res(digit1, NonZeroUsize::from_str),
                tag(" from "),
                map_res(digit1, NonZeroUsize::from_str),
                tag(" to "),
                map_res(digit1, NonZeroUsize::from_str),
            ));

            let (_, (_, num_crates, _, from, _, to)) = tuple_parser.parse(input)?;

            let num_crates = usize::from(num_crates);
            // Go from 1-indexed to 0-indexed
            let from = usize::from(from) - 1;
            let to = usize::from(to) - 1;

            Ok((
                "",
                CrateMove {
                    num_crates,
                    from,
                    to,
                },
            ))
        }
    }

    fn moves_parser<T>() -> impl Parser<T, Vec<CrateMove>, ()>
    where
        T: Iterator<Item = String>,
    {
        move |mut iter: T| {
            let mut moves = vec![];
            while let Some(line) = iter.next() {
                moves.push((move_parser().parse(line.as_str()).unwrap()).1)
            }
            Ok((iter, moves))
        }
    }

    pub fn parser<T>() -> impl Parser<T, (CargoShip, Vec<CrateMove>), ()>
    where
        T: Iterator<Item = String>,
    {
        move |iter: T| {
            // Parse ship
            let (iter, ship) = ship_parser().parse(iter)?;
            // Parse moves
            let (iter, moves) = moves_parser().parse(iter)?;

            Ok((iter, (ship, moves)))
        }
    }
}

pub fn run1<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    use nom::Parser;
    let (_, (mut cargo_ship, crate_moves)) =
        parser::parser().parse(lines).expect("Failed to parse");

    for m in crate_moves {
        cargo_ship.move_crates_9000(&m);
    }
    print!("{}", cargo_ship.get_final_orientation());
}

pub fn run2<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    use nom::Parser;
    let (_, (mut cargo_ship, crate_moves)) =
        parser::parser().parse(lines).expect("Failed to parse");

    for m in crate_moves {
        cargo_ship.move_crates_9001(&m);
    }
    print!("{}", cargo_ship.get_final_orientation());
}
