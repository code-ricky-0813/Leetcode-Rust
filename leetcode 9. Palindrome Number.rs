pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let s = x.to_string();
        let rev: String = s.chars().rev().collect();

        s == rev
    }
}

fn main() {
    let nums = [121, -121, 10];
    for &x in &nums {
        println!("Is {} palindrome? {}", x, Solution::is_palindrome(x));
    }
}
