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


fn main() {
    let content = include_str!("input.txt"); 
    let battery_size = 12;
    let mut battery_sum: i64 = day3_part1(content);
    
    println!("Total joltage from part 1 is: {battery_sum}");

    battery_sum = day3_part2(content, battery_size);
    println!("Total joltage from part 2 is {battery_sum}");
}

fn day3_part2(input_contents: &str, max_battery_amount: usize) -> i64 {
    let mut battery_total: i64 = 0;
     
    input_contents.lines().for_each(|battery_row: &str| {
        let mut local_battery_max: i64 = 0;
        let mut rightmost_digit_index: i8 = -1;
        let mut digits_left = max_battery_amount;
        while digits_left > 0 {
            let mut local_digit_max: i64 = 0;
            let local_rightmost_digit = rightmost_digit_index;
            for (index, value) in battery_row.char_indices().filter(|x| {
                <usize as TryInto<i8>>::try_into(x.0).expect("int conversion") > local_rightmost_digit 
                && <usize as TryInto<i8>>::try_into(x.0).expect("int conversion for current index + digits left") 
                    + <usize as TryInto<i8>>::try_into(digits_left).expect("digits left") <= battery_row.chars().clone().count().try_into().expect("length of string")
            }) {
               if digit_from_char(value) > local_digit_max {
                   local_digit_max = digit_from_char(value);
                   rightmost_digit_index = index.try_into().expect("int conversion of index");
               }
            }
            local_battery_max += local_digit_max;
            if digits_left != 1 {
                local_battery_max *= 10;
            }
            digits_left -= 1;
        }
        println!("Local battery max: {local_battery_max}");
        battery_total += local_battery_max;
    });
    battery_total
}

fn day3_part1(input_contents: &str) -> i64 {
    let mut battery_total: i64 = 0;
    input_contents.lines().for_each(|battery_row: &str| {
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
        if local_max1.unwrap().0 == row_indices.clone().count() {
            println!("Local max 1 at the end, setting it to max 2 and finding new first digit");
            local_max2 = local_max1;
            local_max1 = Some((0, '0'));
            while current_battery != None {
                if current_battery.unwrap().1 > local_max1.unwrap().1 && current_battery.unwrap().0 != local_max2.unwrap().0 {
                    local_max1 = current_battery;
                }
                current_battery = row_indices.next();
            }

        } else {
            println!("Local max 1 not at the end, finding second digit");
            while current_battery != None {
                if current_battery.unwrap().1 > local_max2.unwrap().1 && current_battery.unwrap().0 > local_max1.unwrap().0 {
                    local_max2 = current_battery;
                }
                current_battery = row_indices.next();
            }
        }



        let int_max1 = digit_from_char(local_max1.unwrap().1);
        let int_max2 = digit_from_char(local_max2.unwrap().1);
        let joltage = int_max1 * 10 + int_max2;

        println!("Local max 1: {int_max1}, local max 2: {int_max2}");
        battery_total += joltage;
    });
    battery_total
}

fn digit_from_char(input_char: char) -> i64 {
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
