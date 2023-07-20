/*
9. Palindrome Number
Easy
10.3K
2.5K
Companies

Given an integer x, return true if x is a palindrome , and false otherwise.

Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

Constraints:

    -231 <= x <= 231 - 1
*/

impl solution {
    pub fn is_palindrome(x: i32) -> bool {
        let num_str: String = x.to_string();
        let rev_str: String = num_str.chars().rev().collect();
        if num_str == rev_str {
            return true;
        }
        return false;
    }
}
