use substring::Substring;
use num_traits::pow;

fn main() {
    let input = include_str!("input.txt");
    let ranges = input.split(',');
    let mut total_sum = 0;
    ranges.for_each(|range| {
        //print!("Range: {range}");
        let mut start_end = range.trim_end().split('-');
        let start: i64 = start_end.next().expect("start of range").parse().unwrap();
        //print!(", Start: {start}");
        let end: i64 = start_end.next().expect("end of range").parse().unwrap();
        //println!(", End: {end}");
        total_sum += sum_problematic_ids_part2(start, end);
    });
    println!("Sum of problematic ids: {total_sum}");
}

/*
fn sum_problematic_ids(start: i64, end: i64) -> i64 {
    let mut problematic_id_sum: i64 = 0;
    for id in start..=end {
        let string_id = id.to_string();
        if string_id.len() % 2 == 0 {
            let len = string_id.len();
            if string_id.substring(0, len/2) == string_id.substring(len/2, len) {
                println!("Problematic ID: {string_id}");
                problematic_id_sum += id;
            }
        }
    }
    problematic_id_sum
}
*/

fn sum_problematic_ids_part2(start: i64, end: i64) -> i64 {
    let mut problematic_id_sum: i64 = 0;
    for id in start..=end {
        let string_id = id.to_string();
        let len = string_id.len();
        for substring_size in 1..=len/2 {
            let substring: i64 = string_id.substring(0, substring_size).parse().unwrap();
            if id % substring == 0 {
                let mut id_divided= id/substring;
                let max_power: i32 = id_divided.ilog10().try_into().unwrap();
                let mut current_pow: i32 = 0;
                while current_pow <= max_power {
                    id_divided -= pow(10, current_pow.try_into().unwrap());
                    current_pow += substring_size as i32;
                }
                if id_divided == 0 {
                    println!("Problematic ID: {string_id}");
                    problematic_id_sum += id;
                    break;
                }
            }
        }
    }
    problematic_id_sum
}
