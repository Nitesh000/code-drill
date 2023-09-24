/*First and last occurrences of x
MediumAccuracy: 37.36%Submissions: 239K+Points: 4
Join the most popular course on DSA. Master Skills & Become Employable by enrolling today! 

Given a sorted array arr containing n elements with possibly duplicate is to find indexes of first elements, the task is to find the first and last occurrences of an element x in the given array.
Note: If the number x is not found in the array then return both the indices as -1.

Example 1:

Input:
n=9, x=5
arr[] = { 1, 3, 5, 5, 5, 5, 67, 123, 125 }
Output:  
2 5
Explanation: 
First occurrence of 5 is at index 2 and last occurrence of 5 is at index 5. 

Example 2:

Input:
n=9, x=7
arr[] = { 1, 3, 5, 5, 5, 5, 7, 123, 125 }
Output:  
6 6
Explanation: 
First and last occurrence of 7 is at index 6.

Your Task:
Since, this is a function problem. You don't need to take any input, as it is already accomplished by the driver code. You just need to complete the function find() that takes array arr, integer n and integer x as parameters and returns the required answer.

Expected Time Complexity: O(logN)
Expected Auxiliary Space: O(1).

Constraints:
1 ≤ N ≤ 106
1 ≤ arr[i],x ≤ 109
 * */
class Solution
{
    public:
    vector<int> find(int arr[], int n , int x )
    {
        // code here
        vector<int> result = {-1, -1};
        for (int i = 0; i < n; ++i){
            if (arr[i] == x) {
                if (result.at(0) == -1) {
                     result[0] = i;
                }
                result[1] = i;
            } else if (arr[i] > x) {
                break;
            }
        }
        return result;
    }
};
