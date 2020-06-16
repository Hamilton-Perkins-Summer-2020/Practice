/* Answering Advent of Code 2018 Question 5
  Dave Perkins */
  
use std::io::Read;
use std::fs::File;
use std::iter::Iterator;

fn main() -> std::io::Result<()> {

    // Grab the contents of the input file:
    let mut f = File::open("./src/input.txt").expect("This file does not exist");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("This file cannot be read");

    let polymer = input.as_bytes().to_vec();
    let polymer_copy = polymer.clone();

    part1(polymer)?;
    part2(polymer_copy)?;

    Ok(())
}

fn part1(polymer: Vec<u8>) -> std::io::Result<()> {
    /* Answers part 1 of Question 5 */

    println!("Answer to part 1 is {}", react(polymer).len());
    Ok(())
} 

fn part2(polymer_in: Vec<u8>) -> std::io::Result<()> {
    /* Answers part 2 of Question 5 */

    let polymer = polymer_in;

    // Make a Vec called pairs like [(65, 97), (66, 98), ... , (90, 122)]  
    // that contains the ASCII values of pairs of capitals and smalls, as in 
    // [(A, a), (B, b), ... , (Z, z)]: 
    let capitals: Vec<u8> = (65..91).collect();
    let smalls: Vec<u8> = (97..123).collect();
    let pairs = capitals.iter().zip(smalls);

    // For keeping track of the current best minimum: 
    let mut current_best = polymer.len();

    // Iterate through each pair, removing its entries from polymer, and reacting: 
    for pair in pairs {

        let mut polymer_copy = polymer.clone();
        polymer_copy.retain(|&x| x != *pair.0 && x != pair.1);
        let reduced_length = react(polymer_copy).len();

        // See if current record should be updated:
        if current_best > reduced_length {
            current_best = reduced_length;
        }
    }

    println!("Answer to part 2 is {}", current_best);

    Ok(())
}

fn react(polymer_in: Vec<u8>) -> String {

    let mut polymer = polymer_in;

    loop {
        // Search for two consecutive entries in polymer that differ by 32,
        // which is the separation between capital letters and small letters: 
        let mut reacted = false;

        for k in 0..(polymer.len() - 1) {

            // Check if we can remove consecutive entries:
            if (polymer[k] as i8 - polymer[k+1] as i8).abs() == 32 {
                // We can!
                reacted = true;
                polymer.remove(k);
                polymer.remove(k);
            }

            if reacted {
                break;
            }
        }

        // Quit if we processed the entire polymer without any reactions:
        if !reacted {
            break;
        }

    }

    String::from_utf8(polymer.to_vec()).unwrap()
}