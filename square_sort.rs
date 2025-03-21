impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let mut sq_num = Vec::new();
        
        for &num in nums.iter() {
            sq_num.push(num * num);
        }
        sq_num.sort();
        sq_num
    }
}
