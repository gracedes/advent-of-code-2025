use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::AddAssign;
use std::ops::SubAssign;

fn main() -> io::Result<()> {
    let mut n = 50;
    let mut x = 0;

    let input = File::open("../input.txt")?;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        x.add_assign(turn(&mut n, line?));
    }

    println!("the password is: {x}");
    Ok(())
}

fn turn(n: &mut i32, s: String) -> i32 {
    let first_char = s.chars().nth(0).unwrap();
    let t = &mut s[1..].parse::<i32>().unwrap();
    let mut dx = 0;

    while *t > 0 {
        if first_char == 'L' {
            n.sub_assign(1);
        }
        else {
            n.add_assign(1);
        }

        if modulo(*n, 100) == 0 {
            dx += 1;
        }

        t.sub_assign(1);
    }

    dx
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}