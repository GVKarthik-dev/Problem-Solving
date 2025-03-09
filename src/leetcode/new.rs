/*

3. largest Substring Without Repeating Character

*/


fn solution_1(s: &str) {
    let mut left = 0;
    let n = s.len();
    let mut set:HashSet<i32> = HashSet::new();
    let mut largest = 0;

    for (right, cha) in s.chars().enumerate() {
        while set.contains(&cha) {
            // set.remove(str[right]);
            set.remove(&cha);  // need to learn about this stuff
            left += 1;
        }

        set.insert(cha);
        largest = largest.max(largest, (right - left) + 1);
    }
    largest as i32
}


pub fn main() {
    let input_1 = "abcbbac";
    println!("{}",solution_1(input_1));
}