pub fn get_mean(num: Vec<i32>) -> i32{

    let l = num.len() as i32;
    let mut sum = 0 ;
    for e in num {
        sum += e ;
    }

    sum/l 
}