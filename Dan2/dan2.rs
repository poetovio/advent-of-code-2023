use std::fs;
use std::cmp;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let v: Vec<&str> = input.lines().collect();

    let mut stevec = 0;

    let mut stevec2 = 0;

    for (i, value) in v.iter().enumerate() {

        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;

        let igra = value.split(":").collect::<Vec<&str>>();

        let deli = igra[1].split(";").collect::<Vec<&str>>();

        let mut sestej = true;

        for del in &deli {
            let vrece = del.split(",").collect::<Vec<&str>>();

            for vreca in vrece {
                let beseda = vreca.trim();

                let stevka = beseda.split(" ").collect::<Vec<&str>>();

                match stevka[1] {
                    "red" => {
                        max_red = cmp::max(max_red, stevka[0].parse::<i32>().unwrap());
                        if stevka[0].parse::<i32>().unwrap() > RED {
                            sestej = false;
                        }
                    },
                    "green" => {
                        max_green = cmp::max(max_green, stevka[0].parse::<i32>().unwrap());
                        if stevka[0].parse::<i32>().unwrap() > GREEN {
                            sestej = false;
                        }
                    },
                    "blue" => {
                        max_blue = cmp::max(max_blue, stevka[0].parse::<i32>().unwrap());
                        if stevka[0].parse::<i32>().unwrap() > BLUE {
                            sestej = false;
                        }
                    },
                    _ => (),
                }
            }
        }

        if sestej {
            stevec += i + 1;
        }

        stevec2 += max_red * max_blue * max_green;
    }

    println!("Part 1 -> {}", stevec);
    println!("Part 2 -> {}", stevec2);
}