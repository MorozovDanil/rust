use std::{collections::HashMap, vec};
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashm = HashMap::new(); //create hash map to store numbers and indexes
        for (i,&num) in nums.iter().enumerate() {
            if let Some(&j) = hashm.get(&(target - num)) {
            return vec![j as i32, i as i32]
        }
        hashm.insert(num,i);
    }
    vec![]
}

fn main() {
    two_sum(vec!(1,2,3,4),4);
}

