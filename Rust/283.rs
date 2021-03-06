__________________________________________________________________________________________________
//runtime: 4ms, memory: 2.2MB

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }
        for k in i..nums.len() {
            nums[k] = 0;
        }
    }
}
__________________________________________________________________________________________________

__________________________________________________________________________________________________
