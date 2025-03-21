impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let mut i = 0;
        while i < n {
            if arr[i] == 0 {
                for j in (i + 1..n).rev() {
                    arr[j] = arr[j - 1];
                }
                if i + 1 < n {
                    arr[i + 1] = 0;
                }
                i += 2;
            } else {
                i += 1;
            }
        }
    }
}
