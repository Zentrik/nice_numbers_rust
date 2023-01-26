use malachite::{Natural, num::conversion::traits::Digits};
use malachite::natural::exhaustive::exhaustive_natural_inclusive_range;

fn get_num_uniques_short_circuit(num: &Natural, base: i32) -> bool {
    let mut digits_indicator = vec![false; base as usize];
    let mut number_of_digits = 0;

    let squared = num * num;
    
    for digit in squared.to_digits_asc(&(base as u8)) { 
        if digits_indicator[digit as usize] == true {
            return false
        }

        number_of_digits += 1;

        digits_indicator[digit as usize] = true;
    }

    if number_of_digits > base {
        return false
    }

    let cubed = squared * num;

    for digit in cubed.to_digits_asc(&(base as u8)) { 
        if digits_indicator[digit as usize] == true {
            return false
        }

        number_of_digits += 1;

        digits_indicator[digit as usize] = true;
    }

    if number_of_digits > base {
        return false
    }

    if number_of_digits == base {
        let mut all_the_same = true;

        for digit in digits_indicator {
            if !digit {all_the_same = false}
        }
        if all_the_same {return true} else {return false}
    } else {
        return false
    }
}

// get niceness data on a range of numbers and aggregate it
pub fn search_range(n_start: Natural, n_end: Natural, base: i32, print: bool) -> Vec<Natural> {
    let mut nice_numbers: Vec<Natural> = Vec::new();

    // loop for all items in range (try to optimize anything in here)
    for number in exhaustive_natural_inclusive_range(n_start, n_end) {
        if get_num_uniques_short_circuit(&number, base) {
            if print {
                println!("SUCCESS!!!! A nice number is {number} in base {base}");
            }
            nice_numbers.push(number.clone());
        }
    }

    return nice_numbers
}