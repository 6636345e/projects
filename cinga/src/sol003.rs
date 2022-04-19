fn main(){
	let target: i32 = 9;
	let vec_nums: Vec<i32> = vec![3,3,6];
	two_sum(vec_nums,target);
}

pub fn two_sum(nums: Vec<i32>,target: i32)-> Vec<i32>{
    
        let mut reverse_nums = HashMap::new();

        for (index, value) in nums.iter().enumerate() {
		//dereferencing value so that the actual value can be used
            let num = target - *value;
            if reverse_nums.contains_key(&num) {
                return vec![reverse_nums[&num], index as i32];
            }
            reverse_nums.insert(value, index as i32);
        }
        return vec![];
    }

}
