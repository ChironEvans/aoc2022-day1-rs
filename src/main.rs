use std::{fs, time::Instant};

fn main() {
    let contents = fs::read_to_string("data.txt").expect("Should have been able to read the file");

    let now = Instant::now();

    let elves = split_into_elves(contents.clone());
    let max = elves.iter().fold(std::u32::MIN, |a,b| a.max(*b));

    let elapsed = now.elapsed();
    println!("Elapsed (Pt 1): {:.2?}", elapsed);
    println!("Elf carrying most: {:?}", max);

    let now = Instant::now();

    let mut elves = split_into_elves(contents.clone());
    elves.sort();
    elves.reverse();
    elves.truncate(3);
    let sum_top_3: u32 = elves.iter().sum();

    let elapsed = now.elapsed();
    println!("Elapsed (Pt 2): {:.2?}", elapsed);
    println!("Top 3 elves carrying most: {:?}", sum_top_3);
}

fn split_into_elves(input: String) -> Vec<u32> {
    let elves: Vec<&str> = input.split("\r\n\r\n").collect();
    let totalled_elves: Vec<u32> = elves.into_iter()
    .map(|elf| {
        elf.split("\r\n")
            .map(|food| food.parse::<u32>().unwrap())
            .sum()
    }).collect();

    totalled_elves
}
