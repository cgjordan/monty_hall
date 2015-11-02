extern crate rand;

use rand::Rng;

fn main() {
    let mut stay = 0;
    let mut switch = 0;
    let count = 1000000;
    let talkative = false; // Lots of output slows down the program.
    
    for game in 0..count {
        if talkative { println!("Game number {}", game+1); }
    
        let car = rand::thread_rng().gen_range(1, 4); //upper bound is exclusive
        if talkative { println!("  The car is behind door #{}.", car); }
        
        let pick = rand::thread_rng().gen_range(1, 4);
        if talkative { println!("  The player picks door #{}.", pick); }
        
        // There might be a better way to do this.
        let mut open;
        loop {
            open = rand::thread_rng().gen_range(1, 4);
            if !((open == car) || (open == pick)) { break; }
        }
        if talkative { println!("  The host opens door #{}, revealing a goat behind it.", open); }
        
        // There's got to be a better way to do this.
        let mut offer;
        loop {
            offer = rand::thread_rng().gen_range(1, 4);
            if !((offer == open) || (offer == pick)) { break; }
        }
        if talkative { println!("  The host offers to let the player switch to door #{}.", offer); }
        
        if pick == car {
            stay += 1;
            if talkative { println!("  The door the player picked was the car!"); }
        } else if offer == car {
            switch += 1;
            if talkative { println!("  The door the host offered was the car!"); }
        } else {
            // No-win scenario.
            panic!("Kobayashi_Maru");
        }
    }
    
    println!("Finished simulating {} games.", count);
    println!("Number of cars won by staying:   {}", stay);
    println!("Number of cars won by switching: {}", switch);
}
