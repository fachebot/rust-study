fn u32_sum(nums: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in nums {
        match sum.checked_add(num) {
            None => return None,
            Some(n) => sum = n,
        }
    }
    Some(sum)
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", u32_sum(&nums));

    let nums = vec![4294967295, 2, 3];
    println!("{:?}", u32_sum(&nums));
}
