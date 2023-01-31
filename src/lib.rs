use malachite::{Natural, num::conversion::traits::Digits};
use malachite::natural::exhaustive::exhaustive_natural_inclusive_range;

// use heapless::Vec; // fixed capacity `std::Vec` // Probably only gives 1-2% performance boost


fn is_nice(num: &Natural, base: i32) -> bool {
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

// get the number of unique digits in the concatenated sqube of a specified number
fn get_num_uniques(num: &Natural, base: i32) -> u32 {
    let mut digits_indicator = vec![false; base as usize];

    let squared = num * num;

    for digit in squared.to_digits_asc(&(base as u8)) { 
        digits_indicator[digit as usize] = true;
    }

    let cubed = squared * num;

    for digit in cubed.to_digits_asc(&(base as u8)) { 
        digits_indicator[digit as usize] = true;
    }

    let mut unique_digits = 0;

    for digit in digits_indicator {
        if digit {unique_digits += 1}
    }

    return unique_digits
}

// get niceness data on a range of numbers and aggregate it
pub fn search_range(n_start: Natural, n_end: Natural, base: i32, print: bool) -> std::vec::Vec<Natural> {
    let mut nice_numbers: std::vec::Vec<Natural> = std::vec::Vec::new();

    // loop for all items in range (try to optimize anything in here)
    for number in exhaustive_natural_inclusive_range(n_start, n_end) {
        if is_nice(&number, base) {
        // if get_num_uniques(&number, base) == base as u32 {
            if print {
                println!("SUCCESS!!!! A nice number is {number} in base {base}");
            }
            nice_numbers.push(number.clone());
        }
    }

    return nice_numbers
}