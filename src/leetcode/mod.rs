// use std::os::macos;

pub mod two_sum; //1
pub mod add_two_numbers; //2
pub mod largest_substring_without_repeating_character; // 3
pub mod median_of_two_sorted_arrays; // 4

pub mod single_number; // 136

pub mod contains_duplicate; // 217
pub mod missing_number; // 266
pub mod move_zeroes; // 283

pub mod find_all_numbers_disappeared_in_an_array; // 448

pub mod divide_array_into_equal_pairs; // 2206

pub mod longest_nice_subarray; // 2401

pub mod minimum_cost_walk_in_weighted_graph; //3108
pub mod minimum_operations_to_make_binary_array_elements_equal_to_one_1; // 3191

// need to learn about macros and other
// macro_rules! timeit {
//     ($code:block) => {{
//         let start = Instant::now();
//         let result = $code;
//         let duration = start.elapsed();
//         println!("Execution time: {:?}", duration);
//         result
//     }};
// }