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
            if invalid(i) { acc += i }
        }
    }

    println!("the sum of invalid IDs is: {acc}");
    Ok(())
}

fn invalid(x: u64) -> bool {
    let pow = (x.ilog10() + 1) / 2;
    x / 10u64.pow(pow) == x % 10u64.pow(pow)
}
