pub struct Line {
    pub first: u32,
    pub second: u32,
}

pub fn parse_line(line: &str) -> Line {
    let mut parts = line
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap());

    Line {
        first: parts.next().unwrap(),
        second: parts.next().unwrap(),
    }
}