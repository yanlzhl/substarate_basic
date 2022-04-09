fn main() {
    let vec = vec![999, 1, 3];
    //根据值类型展开打印结果
    match get_sum(&vec) {
        Some(result) => println!("The sum is :{}",result),
        None => println!("The sum is None")
    }
}

//Vec求和
fn get_sum(vec : &[u32]) -> Option<u32> {
    let mut sum:u32 = 0;
    for i in vec {
        match sum.checked_add(*i) {
            Some(v) => {
              sum += *i
            }
            //溢出则直接返回
            None => return None
        };
    }

    Some(sum)
}