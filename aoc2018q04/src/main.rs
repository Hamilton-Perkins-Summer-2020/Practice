/* Answering Advent of Code 2018 Question 4
  Dave Perkins */

// Politely ask the linter to not bug me about dead code while testing:
#![allow(dead_code)]

#[macro_use] // Allows macros from these crates to be used everywhere
extern crate lazy_static;
extern crate regex;

use std::io::Read;
use std::fs::File;
use regex::Regex;

// Used in FromStr trait for the struct Event:
use std::num::ParseIntError;

use std::collections::HashMap;
// use std::cmp::Ordering;
use std::str::FromStr;

fn main() -> std::io::Result<()> {

    // Grab the contents of the input file:
    let mut f = File::open("./src/input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    // Parse the contents of the input file: 
    let mut all_events = create_events(&input);

    // Get answers to both parts:
    both_parts(&mut all_events)?;

    Ok(())
}

fn both_parts(mut events: &mut [Event]) -> std::io::Result<()> {
    /* Answers parts 1 and 2 of Question 4. */

    // Part 1: Determine which guard has spent the most time asleep.

    // Events consist of a Time (year, month, day, hour, minute)
    // and a status ("starts shift", "falls asleep", "wakes up").

    // For checking if an id number is the status of the event:
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]+").unwrap();
    }

    // Keep a list of all guards as a HashMap, mapping id number to minutes asleep:
    let mut guards: HashMap<u32, u32> = HashMap::new();
    let mut current_id = 0;

    // Keep a count for each minute between :00 and :59 that will map to a list
    // of guards that were asleep during that minute: 
    let mut minutes: HashMap<u32, Vec<u32>> = HashMap::new();
    for m in 0..60 {
        minutes.insert(m, vec![]);
    }

    // Process the Events one at a time:
    while events.len() > 0 {

        if RE.is_match(&events[0].status) {
            // A new guard has started a shift:
            current_id = events[0].status.parse().unwrap();
        } else if &events[0].status == "falls asleep" {
            // Determine how long the guard sleeps by comparing to the next event: 
            let sleep_duration = events[1].time.minute - events[0].time.minute;
            // Update the guards HashMap with this information: 
            match guards.get(&current_id) {
                // If the guard is already accounted for, add the sleep_duration:
                Some(&minutes) => &guards.insert(current_id, &minutes + sleep_duration),
                // Otherwise, initiate this guard in the HashMap:
                None => &guards.insert(current_id, sleep_duration),
            };
            // Keep track of each minute corresponding to the guard's nap: 
            for m in events[0].time.minute..events[1].time.minute {
                minutes.entry(m)
                       .or_insert_with(Vec::new)
                       .push(current_id);
            }
        }   // Note: do nothing if status is "wakes up", because we already
            //       dealt with that event during "falls asleep".

        // Dispose of this Event now that we have processed it:
        events = &mut events[1..];
    }

    // Determine which guard (sleepy) has slept the longest: 
    let mut sleepy = 0;
    let mut m = 0;
    for (id, sleep) in &guards {
        if sleep > &m {
            m = *sleep;
            sleepy = *id;
        }
    }

    // Determine during which minute this sleepy guard slept the most: 

    // The entries of the HashMap minutes look like
    //   (11, [13, 45, 45, 90, 13, 13])
    // meaning that guard 13 slept during minute 11 three times, and so on 

    // I only want to search minutes for the guard sleepy, found just above. 
   
    let mut max_minute = 0;
    let mut max_sleep: u32 = 0;
   
    for (key, entry) in &minutes {
        let x: u32 = entry.iter()
                          .filter(|&n| *n == sleepy)
                          .count()
                          as u32;
        if x > max_sleep {
            max_sleep = x;
            max_minute = *key;
        }
    }

    println!("Answer to part 1: {}", sleepy * max_minute);
    // Answer is 118599

    // Part 2: Of all guards, which guard is most frequently asleep on the same minute?

    // I already have the HashMap minutes with entries like
    //   (11, [13, 45, 45, 90, 13, 13])
    // meaning that guard 13 slept during minute 11 three times, and so on 

    // I also have the HashMap guards with entries like
    //   (90, 105)
    // meaning that guard 90 slept for 105 minutes total
    // so I can grab the guard id numbers from there: 

    // Now we iterate through minutes to see how many minutes each guard 
    // slept during it, keeping track of the current record: 

    let mut current_max = 0;
    let mut current_id = 0;
    let mut current_minute = 0;

    for (minute, freqs) in &minutes {
        for (guard_id, _) in &guards {
            let c = freqs.iter().filter(|&x| *guard_id == *x).count() as u32;
            if c > current_max {
                current_minute = *minute;
                current_max = c;
                current_id = *guard_id;
            }
        }
    }

    println!("Answer to part 2: {}", current_minute * current_id);
    // Answer is 33949

    Ok(())
}

fn create_events(input: &str) -> Vec<Event> {
    /* Creates all Event objects from the given input. */

    let mut all_events: Vec<Event> = vec![];

    for line in input.lines() {
        let e = Event::from_str(line).unwrap();
        all_events.push(e);
    }

    // Sort the events by time stamp: 
    all_events.sort_by(|event1, event2| event1.time.cmp(&event2.time));

    all_events
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Time {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
struct Event {
    time: Time,
    status: String, 
}

impl FromStr for Event {
    /* The FromStr method for a struct (in this case, Event) returns an
       instance of Event as generated by a string. In this case, I called
       it using the line:    let e = Event::from_str(line).unwrap();  */

    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Notes: (?x) at the very start of the regex allows 'free spacing', meaning
        // that line returns are ignored; and (?P<the_name>...) captures the expression
        // and the_name will reference that expression. 

        // Every line of input fits this pattern:
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                    \[
                        (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                        \s+
                        (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
                    \]
                    \s+
                    (?:Guard\s\#(?P<id>[0-9]+).+|(?P<status>.+))
                ").unwrap();
        }

        // Use the info in each line of input to create Event objects:
        let caps = RE.captures(s).unwrap();

        let t = Time { year: caps["year"].parse()?,
                    month: caps["month"].parse()?,
                    day: caps["day"].parse()?,
                    hour: caps["hour"].parse()?,
                    minute: caps["minute"].parse()?};

        let s = match caps.name("id") {
            Some(_) => String::from(&caps["id"]),
            None => String::from(&caps["status"]),
        };

        Ok(Event{time: t, status: s})
    }

}