use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut acc: u64 = 0;
    
    let input: String = fs::read_to_string("../input.txt")?;
    let ranges: Vec<_> = input.split(',').collect();

    for range in ranges {
        let bounds: Vec<_> = range.split('-').collect();
        let lower_bound: u64 = bounds[0].parse().unwrap();
        let upper_bound: u64 = bounds[1].trim().parse().unwrap();
        
        for i in lower_bound..=upper_bound {
            if invalid(i.to_string()) { acc += i }
        }
    }

    println!("the sum of invalid IDs is: {acc}");
    Ok(())
}

fn invalid(st: String) -> bool {
    let s = format!("{}.", st);
    for i in 1..=(st.len() / 2) {
        if st.len() % i == 0 {
            let mut matches = 0;
            for j in 1..(st.len() / i) {
                if s[0..i] != s[(i * j)..((i * j) + i)] {break}
                else {matches += 1}
            }
            if matches == (st.len() / i) - 1 {return true;}
        }
    }
    false
}
