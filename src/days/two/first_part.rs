use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn first_part() -> Result<u32, io::Error> {
    let file = File::open("src/days/two/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut reports: u16 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut initial_value: Option<u8> = None;
        let mut order: Option<&str> = None;
        let mut elements = line
            .split_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
            .peekable();

        while let Some(element) = elements.next() {
            if initial_value == None {
                initial_value = Some(element);
            } else {
                if order == None {
                    let score: i8 = initial_value.unwrap() as i8 - element as i8;
                    if score == 0 || score.abs() > 3 {
                        break;
                    }
                    if score < 0 {
                        order = Some("down");
                    }
                    if score > 0 {
                        order = Some("up");
                    }
                    initial_value = Some(element);
                } else {
                    let score: i8 = initial_value.unwrap() as i8 - element as i8;
                    if score == 0 || score.abs() > 3 {
                        break;
                    }
                    if score < 0 && order == Some("up") {
                        break
                    }
                    if score > 0 && order == Some("down") {
                        break
                    }
                    initial_value = Some(element);
                }
            }
            if elements.peek().is_none() {
                reports += 1;
            }
        }
    }

    println!("result: {}", reports);

    Ok(5)
}