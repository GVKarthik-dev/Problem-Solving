// use std::io::{self, Write}; // Import necessary io modules
// use std::process::Command;
// use dotenv::dotenv;
// use std::env;
mod leetcode;
// mod sample;

fn main(){
    println!("Hi Karthik!, What are you doing for today");
    println!("Running main code...");

    //
    // Take input for a string
    // let mut name = String::new();
    // print!("Enter a type: ");
    // io::stdout().flush().unwrap(); // Flush stdout to ensure the prompt is printed
    // io::stdin().read_line(&mut name).unwrap();
    // name = name.trim().to_string(); // Remove any trailing newline characters

    // let output = Command::new("cargo")
    //     .arg("test")
    //     .arg("divide_array_into_equal_pairs") // or specify the test name here
    //     .output()
    //     .expect("Failed to run tests");

    // if name != "check".to_string() {
    //     panic!(" ---> (@ _ @) Are you for real bro! ")
    // }
    //

    // dotenv().ok();

    // match env::var("DATABASE_URL") {
    //     Ok(database_url) => println!("Database URL: {}", database_url),
    //     Err(e) => println!("Couldn't read DATABASE_URL: {}", e),
    // }


    // sample::graph::main();
    // sample::graph::main_2();

    // sample::menu::main();
    // let my_rect:sample::rect::Rect = sample::rect::Rect::new(3, 4);
    // println!("The area for {:?} is {}",my_rect, my_rect.area());
    // sample::main2();
    // sample::rect::main1();

    /*
        -> Leetcode Problems <-
     */

    // leetcode 1st question
    // leetcode::two_sum::main();

    // leetcode 2nd question
    // leetcode::add_two_numbers::main();

    // leetcode 3rd question
    // leetcode::largest_substring_without_repeating_character::main();

    // leetcoce 4th question
    // leetcode::median_of_two_sorted_arrays::main();
    
    // leetcode 136 Single Number
    // leetcode::single_number::main();

    // leetcode 217 question
    // leetcode::contains_duplicate::main();

    // leetcode 266 question
    // leetcode::missing_number::main();

    // leetcode 283. Move Zeroes
    // leetcode::move_zeroes::main()
    
    // leetcode 448 question
    // leetcode::find_all_numbers_disappeared_in_an_array::main()

    // leetcode 977. Squares of a Sorted Array
    // leetcode::squares_of_sorted_array::main();

    // leetcode 2206. Divide Array Into Equal Pairs
    // leetcode::divide_array_into_equal_pairs::main();

    // leetcode 2401. Longest Nice Subarray
    // leetcode::longest_nice_subarray::main();

    // leetcode 3108. Minimum Cost Walk in Weighted Graph
    

    // leetcode 3191.  Minimum Operations to Make Binary Array Elements Equal to One I
    // leetcode::minimum_operations_to_make_binary_array_elements_equal_to_one_1::main();

}