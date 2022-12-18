mod parser {
    use crate::ElfRange;
    use nom::{
        bytes::complete::tag, character::complete::digit1, combinator::map_res,
        sequence::separated_pair, Parser,
    };
    use std::str::FromStr;

    fn range_pair_parser<'a>() -> impl Parser<&'a str, (usize, usize), ()> {
        separated_pair(
            map_res(digit1, usize::from_str),
            tag("-"),
            map_res(digit1, usize::from_str),
        )
    }

    pub fn parser<'a>() -> impl Parser<&'a str, (ElfRange, ElfRange), ()> {
        move |input: &'a str| {
            let (_, (range1, range2)) =
                separated_pair(range_pair_parser(), tag(","), range_pair_parser())(input)?;
            let r1 = ElfRange::from(range1);
            let r2 = ElfRange::from(range2);

            Ok(("", (r1, r2)))
        }
    }
}

pub fn run1<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    use nom::Parser;

    println!(
        "{}",
        lines
            .filter(|line| {
                let (_, (range1, range2)) = parser::parser()
                    .parse(line.as_str())
                    .expect("Failed to parse");
                range1.is_subset(&range2) || range2.is_subset(&range1)
            })
            .count()
    )
}

pub fn run2<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    use nom::Parser;

    println!(
        "{}",
        lines
            .filter(|line| {
                let (_, (range1, range2)) = parser::parser()
                    .parse(line.as_str())
                    .expect("Failed to parse");
                range1.overlaps(&range2)
            })
            .count()
    )
}

pub struct ElfRange {
    start: usize,
    end: usize,
}

impl From<(usize, usize)> for ElfRange {
    fn from((start, end): (usize, usize)) -> Self {
        assert!(start <= end);
        ElfRange { start, end }
    }
}

impl ElfRange {
    fn is_subset(&self, other: &ElfRange) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn overlaps(&self, other: &ElfRange) -> bool {
        let (greater_end, lesser_end) = if self.end > other.end {
            (self, other)
        } else if other.end > self.end {
            (other, self)
        } else {
            return true;
        };

        greater_end.start <= lesser_end.end
    }
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use super::*;

    #[test]
    fn parse_subset() {
        let s = "2-3,2-69";
        let (_, (r1, r2)) = parser::parser().parse(s).unwrap();
        assert!(r1.is_subset(&r2));
    }
}
