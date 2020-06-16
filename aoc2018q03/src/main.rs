/* Answering Advent of Code 2018 Question 3
  Dave Perkins */

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::Read;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {

    // Grab the contents of the input file:
    let mut f = File::open("./src/input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    
    // Create a HashMap that will map locations on the fabric to its number of claims:
    let (fabric, all_claims) = setup(&input);

    // Because I want to use this HashMap for both parts:
    let fabric_clone = fabric.clone();

    // Get answers to both parts:
    part1(fabric)?;
    part2(fabric_clone, all_claims)?;
    Ok(())
}

fn part1(fabric: HashMap<(u32, u32), u32>) -> std::io::Result<()> {
/* Answers part 1 of question 3. Answer is: 110195 */
    
    // Count the cells in the fabric that are claimed by multiple claims:
    let count = fabric.values().filter(|&&c| c > 1).count();
    println!("Answer to part 1: {}", count);

    Ok(())
}

fn part2(fabric: HashMap<(u32, u32), u32>, all_claims: Vec<Claim>) -> std::io::Result<()> {
    /* Answers part 2 of question 3. Answer is: 894 */

    for c in all_claims {
        let mut found = true;
        // Check if this claim overlaps any other claim: 
        for x in c.xloc..(c.xloc + c.width) {
            for y in c.yloc..(c.yloc + c.height) {
                if fabric.get(&(x, y)) != Some(&1) {
                    found = false;
                    break;
                }
            }
        }
        // The question assures us that there's a unique answer:
        if found == true {
            println!("Answer to part 2: {}", c.id);
            break;
        }
    }

    Ok(())
}

fn setup(input: &str) -> (HashMap<(u32, u32), u32>, Vec<Claim>) {
    /* Creates the fabric HashMap and the all_claims vector of Claim objects. */

    // Create a HashMap that will map locations on the fabric to its number of claims:
    let mut fabric: HashMap<(u32, u32), u32> = HashMap::new();

    // Create a list that will contain Claim objects:
    let mut all_claims: Vec<Claim> = Vec::new();

    // Every line of input fits this pattern:
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\#([0-9]+)\s[@]\s([0-9]+)[,]([0-9]+)[:]\s([0-9]+)[x]([0-9]+)$").unwrap();
    }

    for line in input.lines() {
        if line.len() > 0 {
            // Capture the important information for the current claim:
            let caps = RE.captures(line).unwrap();

            let idnum = caps.get(1).map_or("", |m| m.as_str()).parse::<u32>().unwrap();
            let xloc = caps.get(2).map_or("", |m| m.as_str()).parse::<u32>().unwrap();
            let yloc = caps.get(3).map_or("", |m| m.as_str()).parse::<u32>().unwrap();
            let xdim = caps.get(4).map_or("", |m| m.as_str()).parse::<u32>().unwrap();
            let ydim = caps.get(5).map_or("", |m| m.as_str()).parse::<u32>().unwrap();
            
            // Update all_claims and fabric, based on this claim: 
            for x in xloc..(xloc+xdim) {
                for y in yloc..(yloc+ydim) {
                    let new_claim = Claim {
                        id: idnum,
                        xloc: x,
                        yloc: y,
                        width: xdim,
                        height: ydim,
                    };
                    all_claims.push(new_claim);
 
                    match fabric.get(&(x, y)) {
                        // If cell (x, y) already has a claim, add 1 new claim:
                        Some(&number) => &fabric.insert((x, y), &number + 1),
                        // Otherwise, initiate this cell with this 1 claim:
                        None => &fabric.insert((x, y), 1),
                    };
                }
            }
        }
    }

    (fabric, all_claims)
}

#[derive(Debug)] // allows for Claims to be printed
struct Claim {
    id: u32,
    xloc: u32,
    yloc: u32,
    width: u32,
    height: u32,
}