// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use core::panic;


pub fn production_rate_per_hour(speed: u8) -> f64 {
    // to ensure that the variable does not overflow
    let speed_f = speed as f64;
    // success rates
    let rate_zero_to_four:f64 = 1.0;
    let rate_five_to_eight:f64 = 0.9;
    let rate_nine_to_ten:f64 = 0.77;
    // 1 to 4 because the upper bound is not included 
    if  (0..5).contains(&speed) {
        (speed_f * 221.0) * rate_zero_to_four
    }else if (5..9).contains(&speed) {
        (speed_f * 221.0) * rate_five_to_eight
    }else if (9..11).contains(&speed) {
        (speed_f * 221.0) * rate_nine_to_ten
    } else {
        panic!("The speed number is out of range ")
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // to ensure that the variable does not overflow
    let speed = speed as u32;
    // success rates
    let rate_zero_to_four:f32 = 1.0;
    let rate_five_to_eight:f32 = 0.9;
    let rate_nine_to_ten:f32 = 0.77;        
    // 1 to 4 because the upper bound is not included 
    if  (0..5).contains(&speed) {
        ((speed * 221) as f32 * rate_zero_to_four) as u32 / 60
    }else if (5..9).contains(&speed) {
        ((speed * 221) as f32 * 0.9) as u32 / 60 
    }else if (9..11).contains(&speed) {
        ((speed * 221) as f32 * 0.77) as u32 / 60 
    } else {
        panic!("The speed number is out of range ")
    }
}
