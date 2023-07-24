/*
171. Excel Sheet Column Number
Easy
4.3K
340
Companies

Given a string columnTitle that represents the column title as appears in an Excel sheet, return its corresponding column number.

For example:

A -> 1
B -> 2
C -> 3
...
Z -> 26
AA -> 27
AB -> 28
...



Example 1:

Input: columnTitle = "A"
Output: 1

Example 2:

Input: columnTitle = "AB"
Output: 28

Example 3:

Input: columnTitle = "ZY"
Output: 701



Constraints:

    1 <= columnTitle.length <= 7
    columnTitle consists only of uppercase English letters.
    columnTitle is in the range ["A", "FXSHRXW"].

* */

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut page: i32 = 0;
        for c in column_title.chars() {
            page = page * 26 + (c as i32 - 64);
        }
        return page;
    }
}
