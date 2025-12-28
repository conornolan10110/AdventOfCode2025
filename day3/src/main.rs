/*
 *
 * Example input:
 * 987654321111111
 * 811111111111119
 * 234234234234278
 * 818181911112111
 *
 * Get the highest power per row using digits only from left to right
 * e.g. 98, 89, 78, 92
 */

use std::str::FromStr;

macro_rules! concat {
    ($($e:expr),* $(,)?) => { ... };
}


fn main() {
    let content = include_str!("test1.txt");
    let mut battery_sum = 0;
    content.lines().for_each(|battery_row: &str| {
        let mut local_max1: Option<(usize, char)> = Some((0, '0'));
        let mut local_max2: Option<(usize, char)> = Some((0, '0'));
        
        let mut row_indices = battery_row.char_indices();
        let mut current_battery = row_indices.next();
        while current_battery != None {
            if current_battery.unwrap().1 > local_max1.unwrap().1 {
                local_max1 = current_battery;
            }
            current_battery = row_indices.next();
        }
        
        row_indices = battery_row.char_indices();
        current_battery = row_indices.next();
        while current_battery != None {
            if current_battery.unwrap().1 > local_max2.unwrap().1 && current_battery.unwrap().0 > local_max1.unwrap().0 {
                local_max2 = current_battery;
            }
            current_battery = row_indices.next();
        }

        let joltage = i32::from_str(local_max1.unwrap()) * 10 + i32::from_str(local_max2.unwrap());

        battery_sum += joltage;
    });

    println!("Hello, world!");
}












fn digit_from_char(input_char: char) -> i32 {
    match input_char {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("New digit I guess")
   }
}
