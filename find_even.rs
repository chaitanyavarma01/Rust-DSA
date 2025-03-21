impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut even_num = 0;
        for num in nums.iter() {
            let digit_count = num.abs().to_string().len();
            if digit_count % 2 == 0 {
                even_num += 1;
            }
        }
        even_num
    }
}
