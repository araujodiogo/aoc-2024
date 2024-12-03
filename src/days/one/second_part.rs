use std::fs::File;
use std::io::{self, BufRead};
use crate::days::one::models::{parse_line, Line};

pub fn second_part() -> Result<u32, io::Error> {
    let file = File::open("src/days/one/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let Line {first, second} = parse_line(&line?);

        first_list.push(first);
        second_list.push(second);
    }

    let mut similarity_score: u32 = 0;

    for first_list_element in &first_list {
        let mut temp_score: u32 = 0;
        for second_list_element in &second_list {
            if first_list_element == second_list_element {
                temp_score += first_list_element;
            }
        }
        similarity_score += temp_score
    }

    Ok(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_part() {
        match second_part() {
            Ok(value) => assert_eq!(value, 20719933),
            Err(_) => panic!("Expected Ok(5), but got an Err"),
        }
    }
}