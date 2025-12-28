/*
   input.txt has one rotation per line
   L = rotate left (toward lower numbers)
   R = rotate right (toward higher numbers)
   Dial goes from 0 to 99 inclusive
   Inputs are "L25", "R24", etc.
   Right from 99 = 0
   Left from 0 = 99

   Task: Count how many times the dial points to 0 at any point in the rotation sequence


   Cases:
   50 + 100 => +1 to zero_passes
   50 + 25 => +0
   99 + 201 => +3 to zero_passes
   0 - 201 => +3 to zero_passes
   10 - 200 => +2 to zero_passes
   10 - 10 => + 1 to zero_passes

   e.g. 99 + 202 => 301
   301 > 99
   zero_passes = 1
   201 > 99 
   zero_passes = 2
   101 > 99
   zero_passes = 3
   2
   e.g. 3 - 204 => -201
   -201 < 0
   zero_passes = 1
   -101 < 0 
   zero_passes = 2
   -1 < 0 
   zero_passes = 3
   99
   */
const DIAL_SIZE: i32 = 100;

fn main() {
    let mut dial_position = 50;
    let mut zero_lands = 0;
    let mut zero_passes = 0;
    let contents = include_str!("input2.txt");

    contents.lines().for_each(|rotation_input: &str| {
        let starting_position = dial_position;
        let mut final_rotation_amount = parse_rotation(rotation_input);
        //        println!("dial_position: {dial_position}, rotation_instruction: {rotation_input}, rotation_amount: {final_rotation_amount}");
        let spins = final_rotation_amount.abs() / DIAL_SIZE;
        if final_rotation_amount < 0 { 
            final_rotation_amount += DIAL_SIZE * spins;
        }
        if final_rotation_amount >= DIAL_SIZE {
            final_rotation_amount -= DIAL_SIZE * spins;
        }
        zero_passes += spins;

        // Final rotation amount should be within 0 <= rot <= 99

        //        println!("number of rotations: {spins}");
        dial_position += final_rotation_amount;

        if dial_position < 0 {
            dial_position += DIAL_SIZE;
            if starting_position != 0 && dial_position != 0 {
                zero_passes += 1;
                //               println!("adding one more pass");
            }
        }
        if dial_position >= DIAL_SIZE {
            dial_position -= DIAL_SIZE;
            if starting_position != 0 && dial_position != 0 {
                zero_passes += 1;
                //              println!("adding one more pass");
            }
        }

        if dial_position == 0 { 
            zero_lands += 1;
            if starting_position != 0 {
                zero_passes += 1;
                //         println!("dial on 0");
            }
        }
    });

    println!("Final position of dial: {dial_position}, 
        Number of times the dial lands on 0: {zero_lands},
        Number of times the dial passes 0: {zero_passes}");
}

fn parse_rotation(rotation_instruction: &str) -> i32 {
    let rotation_amount: i32 = rotation_instruction[1..].parse().unwrap();
    if rotation_instruction.starts_with("L") {
        -rotation_amount
    } else if rotation_instruction.starts_with("R") {
        rotation_amount 
    } else {
        panic!("Unhandled direction");
    }
}
