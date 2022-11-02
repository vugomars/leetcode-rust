#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn running_sum_1(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter()
            .map(|i| {
                sum += i;
                sum
            })
            .collect()
    }

    pub fn running_sum_2(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |prev, curr| {
                *prev += curr;
                Some(*prev)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn running_man_solution_1() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = Solution::running_sum_1(vec);
        assert_eq!(result, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    pub fn running_man_solution_2() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = Solution::running_sum_2(vec);
        assert_eq!(result, vec![1, 3, 6, 10, 15]);
    }
}
