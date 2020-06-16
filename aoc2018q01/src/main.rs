/* Advent of code 2018 question 1
   Auth: Dave Perkins
   Desc: Solves Day 1: Chronal Calibration
*/

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    for line in input.lines() {
        println!("{}", line);
    }
    Ok(())
}

// match io::stdin().read_line(&mut input) {
//     Ok(n) => {
//         println!("{} bytes read", n);
//         print!("Your name is {}", input);
//     }
//     Err(error) => println!("error: {}", error),
// }
