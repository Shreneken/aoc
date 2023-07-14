use std::cmp::max;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", "./day1_input.txt");

    let contents =
        fs::read_to_string("./day1_input.txt").expect("Should have been able to read the file");
    let mut curr_max = 0;
    let mut curr = 0;
    for text in contents.split('\n') {
        if text == "" {
            curr = 0;
            continue;
        }
        curr += text.parse::<i32>().unwrap();
        curr_max = max(curr, curr_max);
    }
    println!("The Maximum calorie for a single elf was: {curr_max}");
    let mut maxes: [i32; 3] = [0, 0, 0];
    curr = 0;
    let mut i = 1;
    for text in contents.split('\n') {
        if text == "" {
            if curr > maxes[0] {
                maxes[2] = maxes[1];
                maxes[1] = maxes[0];
                maxes[0] = curr;
            } else if curr > maxes[1] {
                maxes[2] = maxes[1];
                maxes[1] = curr;
            } else if curr > maxes[2] {
                maxes[2] = curr;
            }
            curr = 0;
            i+=1;
            println!("{i}. ");
            continue;
        }
        curr += text.parse::<i32>().unwrap();
        println!("3 Maxes are {:?}", maxes);
    }
    let sum = maxes.into_iter().reduce(|a,b| a + b ).unwrap();
    println!("So the sum is {sum}");
}
