use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::AddAssign;
use std::ops::SubAssign;

fn main() -> io::Result<()> {
    let mut n = 50;
    let mut x = 0;

    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        if turn(&mut n, line?) {
            x += 1;
            println!("{n}, {x}")
        }
    }

    println!("the password is: {x}");
    Ok(())
}

fn turn(n: &mut i32, s: String) -> bool {
    let first_char = s.chars().nth(0).unwrap();

    if first_char == 'L' {
        n.sub_assign(&s[1..].parse::<i32>().unwrap());
    }
    else {
        n.add_assign(&s[1..].parse::<i32>().unwrap());
    }

    modulo(*n, 100) == 0
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}