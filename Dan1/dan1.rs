use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {

    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let v: Vec<&str> = input.lines().collect();

    let mut stevec = 0;

    let mut stevec2 = 0;

    let mut numbers_dictionary = HashMap::new();

    let regex_pattern = Regex::new(r#"(zero|one|two|three|four|five|six|seven|eight|nine)"#).unwrap();

    numbers_dictionary.insert("zero", 0);
    numbers_dictionary.insert("one", 1);
    numbers_dictionary.insert("two", 2);
    numbers_dictionary.insert("three", 3);
    numbers_dictionary.insert("four", 4);
    numbers_dictionary.insert("five", 5);
    numbers_dictionary.insert("six", 6);
    numbers_dictionary.insert("seven", 7);
    numbers_dictionary.insert("eight", 8);
    numbers_dictionary.insert("nine", 9);

    for value in v.iter() {
        let mut stevilo = 0;
        let mut stevilo2 = 0;

        let mut string = String::new();
        let mut string2 = String::new();

        for c in value.chars() {
            if c.is_numeric() {
                let stevka = c as i32 - 0x30;
                stevilo += 10 * stevka;
                break;
            }
        }

        for c in value.chars() {
            if c.is_numeric() {
                let stevka = c as i32 - 0x30;
                stevilo2 += 10 * stevka;
                break;
            }

            string.push_str(&String::from(c));

            if let Some(matched) = regex_pattern.find(&String::from(string.clone())) {
                let matched_text = matched.as_str();
        
                if let Some(&number) = numbers_dictionary.get(matched_text) {
                    stevilo2 += 10 * number;
                    break;
                } else {
                    println!("Ustreznega števila ni v HashMapu.");
                }
            } 

        }

        let mut string2 = String::new();

        for c in value.chars().rev() {
            if c.is_numeric() {
                let stevka2 = c as i32 - 0x30;
                stevilo += stevka2;
                break;
            }
        }

        for c in value.chars().rev() {
            if c.is_numeric() {
                let stevka2 = c as i32 - 0x30;
                stevilo2 += stevka2;
                break;
            }

            string2.push_str(&String::from(c));

            let mut obrnjeno = string2.chars().rev().collect::<String>();

            if let Some(matched) = regex_pattern.find(&String::from(obrnjeno.clone())) {
                let matched_text = matched.as_str();
        
                if let Some(&number) = numbers_dictionary.get(matched_text) {
                    stevilo2 +=  number;
                    break;
                } else {
                    println!("Ustreznega števila ni v HashMapu.");
                }
            }
        }

        stevec += stevilo;
        stevec2 += stevilo2;
    }

    println!("Part 1 -> {}", stevec);
    println!("Part 2 -> {}", stevec2)
}