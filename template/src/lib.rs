mod parser {
    use nom::Parser;

    pub fn parser<'a>() -> impl Parser<&'a str, TODO, ()> {
        move |input: &'a str| {}
    }
}

pub fn run1<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    use nom::Parser;

    for line in lines {
        let _todo = parser::parser()
            .parse(line.as_str())
            .expect("Failed to parse");
    }
}

pub fn run2<T>(lines: T)
where
    T: Iterator<Item = String>,
{
    use nom::Parser;

    for line in lines {
        let _todo = parser::parser()
            .parse(line.as_str())
            .expect("Failed to parse");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
