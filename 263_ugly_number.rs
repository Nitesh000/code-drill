/*
263. Ugly Number
Easy
3K
1.6K
Companies

An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.

Given an integer n, return true if n is an ugly number.



Example 1:

Input: n = 6
Output: true
Explanation: 6 = 2 × 3

Example 2:

Input: n = 1
Output: true
Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.

Example 3:

Input: n = 14
Output: false
Explanation: 14 is not ugly since it includes the prime factor 7.



Constraints:

    -231 <= n <= 231 - 1
*/
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n < 0 {
            return false;
        }
        let mut num: i32 = n;
        while num >= 1 {
            if num == 1 {
                return true;
            }
            match num {
                val if val % 2 == 0 => {
                    num /= 2;
                }
                val if val % 3 == 0 => {
                    num /= 3;
                }
                val if val % 5 == 0 => {
                    num /= 5;
                }
                _ => {
                    return false;
                }
            }
        }
        return false;
    }
}
