use std::fs::File;
use std::io::{self, BufRead};
use super::models::Line;
use super::models::parse_line;

fn binary_search_insert(list: &mut Vec<u32>, new_element: u32) {
    match list.binary_search(&new_element) {
        Ok(index) => list.insert(index, new_element),
        Err(index) => list.insert(index, new_element),
    }
}
fn calculate_total_distance(first_list: Vec<u32>, second_list: Vec<u32>) -> u32 {
    let mut total_distance: u32 = 0;
    for (index, element) in first_list.iter().enumerate() {
        total_distance += u32::abs_diff(*element, second_list[index]);
    }
    total_distance
}
pub fn first_part() -> Result<u32, io::Error> {
    let file = File::open("src/days/one/input.txt")?;
    let reader = io::BufReader::new(file);

    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let Line {first, second} = parse_line(&line?);

        binary_search_insert(&mut first_list, first);
        binary_search_insert(&mut second_list, second);
    }

    Ok(calculate_total_distance(first_list, second_list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        match first_part() {
            Ok(value) => assert_eq!(value, 2164381),
            Err(_) => panic!("Expected Ok(2164381), but got an Err"),
        }
    }
}