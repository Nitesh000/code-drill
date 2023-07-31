/*
264. Ugly Number II
Medium
5.6K
272
Companies

An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.

Given an integer n, return the nth ugly number.



Example 1:

Input: n = 10
Output: 12
Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.

Example 2:

Input: n = 1
Output: 1
Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.



Constraints:

    1 <= n <= 1690

* */
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_numbers: Vec<i32> = Vec::new();
        ugly_numbers.push(1);
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;
        let mut next_multiple_of_2 = 2;
        let mut next_multiple_of_3 = 3;
        let mut next_multiple_of_5 = 5;
        let mut next_ugly_number = 1;
        for _i in 1..n {
            next_ugly_number = std::cmp::min(
                next_multiple_of_2,
                std::cmp::min(next_multiple_of_3, next_multiple_of_5),
            );
            ugly_numbers.push(next_ugly_number);
            if next_ugly_number == next_multiple_of_2 {
                i2 += 1;
                next_multiple_of_2 = ugly_numbers[i2] * 2;
            }
            if next_ugly_number == next_multiple_of_3 {
                i3 += 1;
                next_multiple_of_3 = ugly_numbers[i3] * 3;
            }
            if next_ugly_number == next_multiple_of_5 {
                i5 += 1;
                next_multiple_of_5 = ugly_numbers[i5] * 5;
            }
        }
        next_ugly_number
    }
}
