// https://leetcode.com/problems/number-of-1-bits/
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}
