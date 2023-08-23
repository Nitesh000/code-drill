/*
1013. Partition Array Into Three Parts With Equal Sum
Easy
1.6K
143
Companies

Given an array of integers arr, return true if we can partition the array into three non-empty parts with equal sums.

Formally, we can partition the array if we can find indexes i + 1 < j with (arr[0] + arr[1] + ... + arr[i] == arr[i + 1] + arr[i + 2] + ... + arr[j - 1] == arr[j] + arr[j + 1] + ... + arr[arr.length - 1])



Example 1:

Input: arr = [0,2,1,-6,6,-7,9,1,2,0,1]
Output: true
Explanation: 0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1

Example 2:

Input: arr = [0,2,1,-6,6,7,9,-1,2,0,1]
Output: false

Example 3:

Input: arr = [3,3,6,5,-2,2,5,1,-9,4]
Output: true
Explanation: 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4



Constraints:

    3 <= arr.length <= 5 * 104
    -104 <= arr[i] <= 104

*/
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        let oneThird = sum / 3;
        let mut i1 = -1;
        let mut i2 = -1;

        let mut curSum = 0;
        for i in 0..arr.len() {
            curSum += arr[i];
            if i1 == -1 && curSum == oneThird {
                i1 = i as i32;
            } else if i1 != -1 && curSum == oneThird * 2 {
                i2 = i as i32;
            }
            if i1 != -1 && i2 != -1 && i2 != (arr.len() as i32 - 1) {
                return true;
            }
        }
        return false;
    }
}
