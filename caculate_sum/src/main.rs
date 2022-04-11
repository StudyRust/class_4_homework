fn sum_for_u32(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for num in arr {
        match sum.checked_add(*num) {
            Some(tmp) => sum = tmp,
            None => return None,
        }
    }
    Some(sum)
}

// 4294967295
fn main() {
    let arr: Vec<u32> = vec!(1,2,3,4,5);
    println!("{:?}", sum_for_u32(&arr));
    let arr: Vec<u32> = vec!(1,2,3,4,4294967295);
    println!("{:?}", sum_for_u32(&arr));
}
